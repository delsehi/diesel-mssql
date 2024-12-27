use super::super::backend::MssqlSelectSyntax;
use super::Mssql;
use diesel::query_builder::BoxedLimitOffsetClause;
use diesel::query_builder::BoxedSelectStatement;
use diesel::query_builder::QueryFragment;
use diesel::query_builder::SelectStatement;
use diesel::query_builder::*;

trait MssqlOrderLimitOffsetHelper<Order, LimitOffset> {
    fn walk_ast<'b>(
        &'b self,
        out: diesel::query_builder::AstPass<'_, 'b, Mssql>,
    ) -> diesel::QueryResult<()>;
}

impl<F, S, D, W, G, H, O, LC>
    MssqlOrderLimitOffsetHelper<OrderClause<O>, LimitOffsetClause<NoLimitClause, NoOffsetClause>>
    for SelectStatement<
        F,
        S,
        D,
        W,
        OrderClause<O>,
        LimitOffsetClause<NoLimitClause, NoOffsetClause>,
        G,
        H,
        LC,
    >
where
    F: QueryFragment<Mssql>,
    S: QueryFragment<Mssql>,
    D: QueryFragment<Mssql>,
    W: QueryFragment<Mssql>,
    OrderClause<O>: QueryFragment<Mssql>,
    LimitOffsetClause<NoLimitClause, NoOffsetClause>: QueryFragment<Mssql>,
    G: QueryFragment<Mssql>,
    H: QueryFragment<Mssql>,
    LC: QueryFragment<Mssql>,
{
    fn walk_ast<'b>(
        &'b self,
        mut out: diesel::query_builder::AstPass<'_, 'b, Mssql>,
    ) -> diesel::QueryResult<()> {
        out.push_sql("SELECT ");
        self.distinct.walk_ast(out.reborrow())?;
        self.select.walk_ast(out.reborrow())?;
        self.from.walk_ast(out.reborrow())?;
        self.where_clause.walk_ast(out.reborrow())?;
        self.group_by.walk_ast(out.reborrow())?;
        self.having.walk_ast(out.reborrow())?;
        self.order.walk_ast(out.reborrow())?;
        self.limit_offset.walk_ast(out.reborrow())?;
        Ok(())
    }
}

impl<F, S, D, W, G, H, L, O, LC>
    MssqlOrderLimitOffsetHelper<OrderClause<O>, LimitOffsetClause<LimitClause<L>, NoOffsetClause>>
    for SelectStatement<
        F,
        S,
        D,
        W,
        OrderClause<O>,
        LimitOffsetClause<LimitClause<L>, NoOffsetClause>,
        G,
        H,
        LC,
    >
where
    F: QueryFragment<Mssql>,
    S: QueryFragment<Mssql>,
    D: QueryFragment<Mssql>,
    W: QueryFragment<Mssql>,
    OrderClause<O>: QueryFragment<Mssql>,
    LimitOffsetClause<LimitClause<L>, NoOffsetClause>: WalkAsTop,
    G: QueryFragment<Mssql>,
    H: QueryFragment<Mssql>,
    LC: QueryFragment<Mssql>,
{
    fn walk_ast<'b>(
        &'b self,
        mut out: diesel::query_builder::AstPass<'_, 'b, Mssql>,
    ) -> diesel::QueryResult<()> {
        out.push_sql("SELECT ");
        self.distinct.walk_ast(out.reborrow())?;
        self.limit_offset.walk_ast_as_top(out.reborrow())?;
        self.select.walk_ast(out.reborrow())?;
        self.from.walk_ast(out.reborrow())?;
        self.where_clause.walk_ast(out.reborrow())?;
        self.group_by.walk_ast(out.reborrow())?;
        self.having.walk_ast(out.reborrow())?;
        self.order.walk_ast(out.reborrow())?;
        Ok(())
    }
}

impl<F, S, D, W, G, H, L, O, Of, LC>
    MssqlOrderLimitOffsetHelper<OrderClause<O>, LimitOffsetClause<LimitClause<L>, OffsetClause<Of>>>
    for SelectStatement<
        F,
        S,
        D,
        W,
        OrderClause<O>,
        LimitOffsetClause<LimitClause<L>, OffsetClause<Of>>,
        G,
        H,
        LC,
    >
where
    F: QueryFragment<Mssql>,
    S: QueryFragment<Mssql>,
    D: QueryFragment<Mssql>,
    W: QueryFragment<Mssql>,
    OrderClause<O>: QueryFragment<Mssql>,
    LimitOffsetClause<LimitClause<L>, OffsetClause<Of>>: QueryFragment<Mssql>,
    G: QueryFragment<Mssql>,
    H: QueryFragment<Mssql>,
    LC: QueryFragment<Mssql>,
{
    fn walk_ast<'b>(
        &'b self,
        mut out: diesel::query_builder::AstPass<'_, 'b, Mssql>,
    ) -> diesel::QueryResult<()> {
        out.push_sql("SELECT ");
        self.distinct.walk_ast(out.reborrow())?;
        self.select.walk_ast(out.reborrow())?;
        self.from.walk_ast(out.reborrow())?;
        self.where_clause.walk_ast(out.reborrow())?;
        self.group_by.walk_ast(out.reborrow())?;
        self.having.walk_ast(out.reborrow())?;
        self.order.walk_ast(out.reborrow())?;
        self.limit_offset.walk_ast(out.reborrow())?;
        Ok(())
    }
}

impl<F, S, D, W, G, H, LC>
    MssqlOrderLimitOffsetHelper<NoOrderClause, LimitOffsetClause<NoLimitClause, NoOffsetClause>>
    for SelectStatement<
        F,
        S,
        D,
        W,
        NoOrderClause,
        LimitOffsetClause<NoLimitClause, NoOffsetClause>,
        G,
        H,
        LC,
    >
where
    F: QueryFragment<Mssql>,
    S: QueryFragment<Mssql>,
    D: QueryFragment<Mssql>,
    W: QueryFragment<Mssql>,
    NoOrderClause: QueryFragment<Mssql>,
    LimitOffsetClause<NoLimitClause, NoOffsetClause>: QueryFragment<Mssql>,
    G: QueryFragment<Mssql>,
    H: QueryFragment<Mssql>,
    LC: QueryFragment<Mssql>,
{
    fn walk_ast<'b>(
        &'b self,
        mut out: diesel::query_builder::AstPass<'_, 'b, Mssql>,
    ) -> diesel::QueryResult<()> {
        out.push_sql("SELECT ");
        self.distinct.walk_ast(out.reborrow())?;
        self.select.walk_ast(out.reborrow())?;
        self.from.walk_ast(out.reborrow())?;
        self.where_clause.walk_ast(out.reborrow())?;
        self.group_by.walk_ast(out.reborrow())?;
        self.having.walk_ast(out.reborrow())?;
        self.order.walk_ast(out.reborrow())?;
        self.limit_offset.walk_ast(out.reborrow())?;
        Ok(())
    }
}

impl<F, S, D, W, G, H, L, LC>
    MssqlOrderLimitOffsetHelper<NoOrderClause, LimitOffsetClause<LimitClause<L>, NoOffsetClause>>
    for SelectStatement<
        F,
        S,
        D,
        W,
        NoOrderClause,
        LimitOffsetClause<LimitClause<L>, NoOffsetClause>,
        G,
        H,
        LC,
    >
where
    F: QueryFragment<Mssql>,
    S: QueryFragment<Mssql>,
    D: QueryFragment<Mssql>,
    W: QueryFragment<Mssql>,
    NoOrderClause: QueryFragment<Mssql>,
    LimitOffsetClause<LimitClause<L>, NoOffsetClause>: WalkAsTop,
    G: QueryFragment<Mssql>,
    H: QueryFragment<Mssql>,
    LC: QueryFragment<Mssql>,
{
    fn walk_ast<'b>(
        &'b self,
        mut out: diesel::query_builder::AstPass<'_, 'b, Mssql>,
    ) -> diesel::QueryResult<()> {
        out.push_sql("SELECT ");
        self.distinct.walk_ast(out.reborrow())?;
        self.limit_offset.walk_ast_as_top(out.reborrow())?;
        self.select.walk_ast(out.reborrow())?;
        self.from.walk_ast(out.reborrow())?;
        self.where_clause.walk_ast(out.reborrow())?;
        self.group_by.walk_ast(out.reborrow())?;
        self.having.walk_ast(out.reborrow())?;
        self.order.walk_ast(out.reborrow())?;
        Ok(())
    }
}

pub trait WalkAsTop {
    fn walk_ast_as_top<'b>(
        &'b self,
        out: diesel::query_builder::AstPass<'_, 'b, Mssql>,
    ) -> diesel::QueryResult<()>;
}

impl<L> WalkAsTop for LimitOffsetClause<LimitClause<L>, NoOffsetClause>
where
    L: QueryFragment<Mssql>,
{
    fn walk_ast_as_top<'b>(
        &'b self,
        mut out: diesel::query_builder::AstPass<'_, 'b, Mssql>,
    ) -> diesel::QueryResult<()> {
        out.push_sql(" TOP(");
        self.limit_clause.0.walk_ast(out.reborrow())?;
        out.push_sql(") ");
        Ok(())
    }
}

impl<F, S, D, W, O, LOf, G, H, LC> QueryFragment<Mssql, MssqlSelectSyntax>
    for SelectStatement<F, S, D, W, O, LOf, G, H, LC>
where
    Self: MssqlOrderLimitOffsetHelper<O, LOf>,
{
    fn walk_ast<'b>(
        &'b self,
        out: diesel::query_builder::AstPass<'_, 'b, Mssql>,
    ) -> diesel::QueryResult<()> {
        <Self as MssqlOrderLimitOffsetHelper<O, LOf>>::walk_ast(self, out)
    }
}

impl<'a, S, QS, GB> QueryFragment<Mssql, MssqlSelectSyntax>
    for BoxedSelectStatement<'a, S, QS, Mssql, GB>
where
    BoxedLimitOffsetClause<'a, Mssql>: QueryFragment<Mssql>,
    QS: QueryFragment<Mssql>,
{
    fn walk_ast<'b>(
        &'b self,
        mut out: diesel::query_builder::AstPass<'_, 'b, Mssql>,
    ) -> diesel::QueryResult<()> {
        out.push_sql("SELECT ");
        self.distinct.walk_ast(out.reborrow())?;
        self.select.walk_ast(out.reborrow())?;
        self.from.walk_ast(out.reborrow())?;
        self.where_clause.walk_ast(out.reborrow())?;
        self.group_by.walk_ast(out.reborrow())?;
        self.having.walk_ast(out.reborrow())?;
        match self.order {
            Some(ref order) => {
                out.push_sql(" ORDER BY ");
                order.walk_ast(out.reborrow())?;
            },
            // if we have no order clause but a limit/offset clause
            // we need to generate a fake order statement
            None if self.limit_offset.limit.is_some() || self.limit_offset.offset.is_some() => {
                // we don't have any reasonable thing to order by
                // so we just order by the first column in the select clause
                out.push_sql(" ORDER BY 0 ");
            }
            None => {}
        }
        self.limit_offset.walk_ast(out.reborrow())?;
        Ok(())
    }
}
