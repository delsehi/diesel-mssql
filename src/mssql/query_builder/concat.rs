use super::super::backend::ConcatWithPlusClause;
use super::super::backend::Mssql;
use diesel::{expression::Concat, query_builder::QueryFragment};

impl<L, R> QueryFragment<Mssql, ConcatWithPlusClause> for Concat<L, R>
where
    L: QueryFragment<Mssql>,
    R: QueryFragment<Mssql>,
{
    fn walk_ast<'b>(
        &'b self,
        mut out: diesel::query_builder::AstPass<'_, 'b, Mssql>,
    ) -> diesel::result::QueryResult<()> {
        // TODO, use CONCAT() like MySQL instead.
        out.push_sql("(");
        self.left.walk_ast(out.reborrow())?;
        out.push_sql(" + ");
        self.right.walk_ast(out.reborrow())?;
        out.push_sql(")");
        Ok(())
    }
}
