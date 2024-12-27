use diesel::connection::{
    Connection, InTransactionStatus, TransactionDepthChange, TransactionManager,
    TransactionManagerStatus, ValidTransactionManagerStatus,
};
use diesel::result::{Error, QueryResult};
use std::borrow::Cow;
use std::num::NonZeroU32;

/// A slightly modified copy of the AnsiTransactionManager.
#[derive(Default, Debug)]
pub struct MssqlTransactionManager {
    pub(crate) status: TransactionManagerStatus,
}

impl MssqlTransactionManager {
    fn get_transaction_state<Conn>(
        conn: &mut Conn,
    ) -> QueryResult<&mut ValidTransactionManagerStatus>
    where
        Conn: Connection<TransactionManager = Self>,
    {
        conn.transaction_state().status.transaction_state()
    }

    /// Begin a transaction with custom SQL
    ///
    /// This is used by connections to implement more complex transaction APIs
    /// to set things such as isolation levels.
    /// Returns an error if already inside of a transaction.
    pub fn begin_transaction_sql<Conn>(conn: &mut Conn, sql: &str) -> QueryResult<()>
    where
        Conn: Connection<TransactionManager = Self>,
    {
        let state = Self::get_transaction_state(conn)?;
        match state.transaction_depth() {
            None => {
                conn.batch_execute(sql)?;
                Self::get_transaction_state(conn)?
                    .change_transaction_depth(TransactionDepthChange::IncreaseDepth)?;
                Ok(())
            }
            Some(_depth) => Err(Error::AlreadyInTransaction),
        }
    }
}

impl<Conn> TransactionManager<Conn> for MssqlTransactionManager
where
    Conn: Connection<TransactionManager = Self>,
{
    type TransactionStateData = Self;

    fn begin_transaction(conn: &mut Conn) -> QueryResult<()> {
        let transaction_state = Self::get_transaction_state(conn)?;
        let transaction_depth = transaction_state.transaction_depth();
        let start_transaction_sql = match transaction_depth {
            None => Cow::from("BEGIN TRANSACTION"),
            Some(transaction_depth) => Cow::from(format!(
                "SAVE TRANSACTION diesel_savepoint_{transaction_depth}"
            )),
        };
        diesel::connection::InstrumentationEvent::begin_transaction(
            NonZeroU32::new(transaction_depth.map_or(0, NonZeroU32::get).wrapping_add(1)).unwrap(),
        );
        conn.batch_execute(&start_transaction_sql)?;
        Self::get_transaction_state(conn)?
            .change_transaction_depth(TransactionDepthChange::IncreaseDepth)?;

        Ok(())
    }

    fn rollback_transaction(conn: &mut Conn) -> QueryResult<()> {
        let transaction_state = Self::get_transaction_state(conn)?;

        let (
            (rollback_sql, rolling_back_top_level),
            requires_rollback_maybe_up_to_top_level_before_execute,
        ) = match transaction_state.in_transaction {
            Some(ref in_transaction) => (
                match in_transaction.transaction_depth.get() {
                    1 => (Cow::Borrowed("ROLLBACK TRANSACTION"), true),
                    depth_gt1 => (
                        Cow::Owned(format!(
                            "ROLLBACK TRANSACTION diesel_savepoint_{}",
                            depth_gt1 - 1
                        )),
                        false,
                    ),
                },
                in_transaction.requires_rollback_maybe_up_to_top_level,
            ),
            None => return Err(Error::NotInTransaction),
        };
        let depth = transaction_state
            .transaction_depth()
            .expect("We know that we are in a transaction here");
        diesel::connection::InstrumentationEvent::rollback_transaction(
            NonZeroU32::new(depth.into()).unwrap(),
        );

        match conn.batch_execute(&rollback_sql) {
            Ok(()) => {
                match Self::get_transaction_state(conn)?
                    .change_transaction_depth(TransactionDepthChange::DecreaseDepth)
                {
                    Ok(()) => {}
                    Err(Error::NotInTransaction) if rolling_back_top_level => {
                        // Transaction exit may have already been detected by connection
                        // implementation. It's fine.
                    }
                    Err(e) => return Err(e),
                }
                Ok(())
            }
            Err(rollback_error) => {
                let tm_status = Self::transaction_manager_status_mut(conn);
                match tm_status {
                    TransactionManagerStatus::Valid(ValidTransactionManagerStatus {
                        in_transaction:
                            Some(InTransactionStatus {
                                transaction_depth,
                                requires_rollback_maybe_up_to_top_level,
                                ..
                            }),
                        ..
                    }) if transaction_depth.get() > 1 => {
                        // A savepoint failed to rollback - we may still attempt to repair
                        // the connection by rolling back higher levels.

                        // To make it easier on the user (that they don't have to really
                        // look at actual transaction depth and can just rely on the number
                        // of times they have called begin/commit/rollback) we still
                        // decrement here:
                        *transaction_depth = NonZeroU32::new(transaction_depth.get() - 1)
                            .expect("Depth was checked to be > 1");
                        *requires_rollback_maybe_up_to_top_level = true;
                        if requires_rollback_maybe_up_to_top_level_before_execute {
                            // In that case, we tolerate that savepoint releases fail
                            // -> we should ignore errors
                            return Ok(());
                        }
                    }
                    TransactionManagerStatus::Valid(ValidTransactionManagerStatus {
                        in_transaction: None,
                        ..
                    }) => {
                        // we would have returned `NotInTransaction` if that was already the state
                        // before we made our call
                        // => Transaction manager status has been fixed by the underlying connection
                        // so we don't need to set_in_error
                    }
                    _ => tm_status.set_in_error(),
                }
                Err(rollback_error)
            }
        }
    }

    /// If the transaction fails to commit due to a `SerializationFailure` or a
    /// `ReadOnlyTransaction` a rollback will be attempted. If the rollback succeeds,
    /// the original error will be returned, otherwise the error generated by the rollback
    /// will be returned. In the second case the connection will be considered broken
    /// as it contains a uncommitted unabortable open transaction.
    fn commit_transaction(conn: &mut Conn) -> QueryResult<()> {
        let transaction_state = Self::get_transaction_state(conn)?;
        let transaction_depth = transaction_state.transaction_depth();
        let (commit_sql, committing_top_level) = match transaction_depth {
            None => return Err(Error::NotInTransaction),
            Some(transaction_depth) if transaction_depth.get() == 1 => {
                (Cow::Borrowed("COMMIT TRANSACTION"), true)
            }
            Some(transaction_depth) => (
                // TODO: Investigate why a test fails because of this.
                Cow::Owned(format!(
                    "COMMIT TRANSACTION diesel_savepoint_{}",
                    transaction_depth.get() - 1
                )),
                false,
            ),
            _ => panic!("Wrong transaction depth?"),
        };
        let depth = transaction_state
            .transaction_depth()
            .expect("We know that we are in a transaction here");
        diesel::connection::InstrumentationEvent::commit_transaction(
            NonZeroU32::new(depth.into()).unwrap(),
        );
        match conn.batch_execute(&commit_sql) {
            Ok(()) => {
                match Self::get_transaction_state(conn)?
                    .change_transaction_depth(TransactionDepthChange::DecreaseDepth)
                {
                    Ok(()) => {}
                    Err(Error::NotInTransaction) if committing_top_level => {
                        // Transaction exit may have already been detected by connection.
                        // It's fine
                    }
                    Err(e) => return Err(e),
                }
                Ok(())
            }
            Err(commit_error) => {
                if let TransactionManagerStatus::Valid(ValidTransactionManagerStatus {
                    in_transaction:
                        Some(InTransactionStatus {
                            requires_rollback_maybe_up_to_top_level: true,
                            ..
                        }),
                    ..
                }) = conn.transaction_state().status
                {
                    match Self::rollback_transaction(conn) {
                        Ok(()) => {}
                        Err(rollback_error) => {
                            conn.transaction_state().status.set_in_error();
                            return Err(Error::RollbackErrorOnCommit {
                                rollback_error: Box::new(rollback_error),
                                commit_error: Box::new(commit_error),
                            });
                        }
                    }
                }
                Err(commit_error)
            }
        }
    }

    fn transaction_manager_status_mut(conn: &mut Conn) -> &mut TransactionManagerStatus {
        &mut conn.transaction_state().status
    }
}
