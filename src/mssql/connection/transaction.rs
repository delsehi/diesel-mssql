use diesel::connection::{
    TransactionManager, TransactionManagerStatus, ValidTransactionManagerStatus,
};

use super::MssqlConnection;

pub struct MssqlTransactionManager;

impl TransactionManager<MssqlConnection> for MssqlTransactionManager {
    type TransactionStateData = Self;

    fn begin_transaction(conn: &mut MssqlConnection) -> diesel::QueryResult<()> {
        todo!()
    }

    fn rollback_transaction(conn: &mut MssqlConnection) -> diesel::QueryResult<()> {
        todo!()
    }

    fn commit_transaction(conn: &mut MssqlConnection) -> diesel::QueryResult<()> {
        todo!()
    }

    fn transaction_manager_status_mut(conn: &mut MssqlConnection) -> &mut TransactionManagerStatus {
        todo!()
    }

    fn transaction<F, R, E>(conn: &mut MssqlConnection, callback: F) -> Result<R, E>
    where
        F: FnOnce(&mut MssqlConnection) -> Result<R, E>,
        E: From<diesel::result::Error>,
    {
        Self::begin_transaction(conn)?;
        match callback(&mut *conn) {
            Ok(value) => {
                Self::commit_transaction(conn)?;
                Ok(value)
            }
            Err(user_error) => match Self::rollback_transaction(conn) {
                Ok(()) => Err(user_error),
                Err(diesel::result::Error::BrokenTransactionManager) => {
                    // In this case we are probably more interested by the
                    // original error, which likely caused this
                    Err(user_error)
                }
                Err(rollback_error) => Err(rollback_error.into()),
            },
        }
    }

    fn is_broken_transaction_manager(conn: &mut MssqlConnection) -> bool {
        //TODO! Implement this.
        false
        // match Self::transaction_manager_status_mut(conn).transaction_state() {
        //     Ok(ValidTransactionManagerStatus {
        //         in_transaction: None,
        //     }) => false,
        //     Err(_) => true,
        //     Ok(ValidTransactionManagerStatus {
        //         in_transaction: Some(s),
        //     }) => !s.test_transaction,
        // }
    }
}
