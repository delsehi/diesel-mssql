use super::super::backend::Mssql;
use diesel::{query_builder::*, QueryResult};

impl QueryFragment<Mssql> for LimitOffsetClause<NoLimitClause, NoOffsetClause> {
    fn walk_ast<'b>(&'b self, _out: AstPass<'_, 'b, Mssql>) -> diesel::QueryResult<()> {
        Ok(())
    }
}

impl<L, O> QueryFragment<Mssql> for LimitOffsetClause<LimitClause<L>, OffsetClause<O>>
where
    L: QueryFragment<Mssql>,
    O: QueryFragment<Mssql>,
{
    fn walk_ast<'b>(
        &'b self,
        mut out: diesel::query_builder::AstPass<'_, 'b, Mssql>,
    ) -> diesel::QueryResult<()> {
        out.push_sql(" OFFSET ");
        self.offset_clause.0.walk_ast(out.reborrow())?;
        out.push_sql(" ROWS FETCH NEXT ");
        self.limit_clause.0.walk_ast(out.reborrow())?;
        out.push_sql(" ROWS ONLY ");
        Ok(())
    }
}

impl<L> QueryFragment<Mssql> for LimitOffsetClause<LimitClause<L>, NoOffsetClause>
where
    L: QueryFragment<Mssql>,
{
    fn walk_ast<'b>(
        &'b self,
        mut out: diesel::query_builder::AstPass<'_, 'b, Mssql>,
    ) -> diesel::QueryResult<()> {
        out.push_sql(" TOP(");
        self.limit_clause.0.walk_ast(out.reborrow())?;
        out.push_sql(") ");
        Ok(())
    }
}

impl QueryFragment<Mssql> for BoxedLimitOffsetClause<'_, Mssql> {
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Mssql>) -> QueryResult<()> {
        match (self.limit.as_ref(), self.offset.as_ref()) {
            (Some(limit), Some(offset)) => {
                out.push_sql(" OFFSET ");
                offset.walk_ast(out.reborrow())?;
                out.push_sql(" ROW FETCH NEXT ");
                limit.walk_ast(out.reborrow())?;
                out.push_sql(" ROWS ONLY ");
            }
            (Some(limit), None) => {
                out.push_sql(" OFFSET 0 ROW FETCH NEXT ");
                limit.walk_ast(out.reborrow())?;
                out.push_sql(" ROWS ONLY ");
            }
            (None, Some(offset)) => {
                out.push_sql(" OFFSET ");
                offset.walk_ast(out.reborrow())?;
            }
            (None, None) => {}
        }
        Ok(())
    }
}

impl<'a> IntoBoxedClause<'a, Mssql> for LimitOffsetClause<NoLimitClause, NoOffsetClause> {
    type BoxedClause = BoxedLimitOffsetClause<'a, Mssql>;

    fn into_boxed(self) -> Self::BoxedClause {
        BoxedLimitOffsetClause {
            limit: None,
            offset: None,
        }
    }
}

impl<'a, L> IntoBoxedClause<'a, Mssql> for LimitOffsetClause<LimitClause<L>, NoOffsetClause>
where
    L: QueryFragment<Mssql> + Send + 'a,
{
    type BoxedClause = BoxedLimitOffsetClause<'a, Mssql>;

    fn into_boxed(self) -> Self::BoxedClause {
        BoxedLimitOffsetClause {
            limit: Some(Box::new(self.limit_clause.0)),
            offset: None,
        }
    }
}

impl<'a, O> IntoBoxedClause<'a, Mssql> for LimitOffsetClause<NoLimitClause, OffsetClause<O>>
where
    O: QueryFragment<Mssql> + Send + 'a,
{
    type BoxedClause = BoxedLimitOffsetClause<'a, Mssql>;

    fn into_boxed(self) -> Self::BoxedClause {
        BoxedLimitOffsetClause {
            limit: None,
            offset: Some(Box::new(self.offset_clause.0)),
        }
    }
}

impl<'a, L, O> IntoBoxedClause<'a, Mssql> for LimitOffsetClause<LimitClause<L>, OffsetClause<O>>
where
    L: QueryFragment<Mssql> + Send + 'a,
    O: QueryFragment<Mssql> + Send + 'a,
{
    type BoxedClause = BoxedLimitOffsetClause<'a, Mssql>;

    fn into_boxed(self) -> Self::BoxedClause {
        BoxedLimitOffsetClause {
            limit: Some(Box::new(self.limit_clause.0)),
            offset: Some(Box::new(self.offset_clause.0)),
        }
    }
}
