use diesel::query_builder::LimitOffsetClause;
use diesel::query_builder::QueryBuilder;
use diesel::query_builder::QueryFragment;

use super::backend::Mssql;

pub struct MssqlQueryBuilder {
    query: String,
    bind_idx: u32,
}

impl MssqlQueryBuilder {
    pub fn new() -> Self {
        MssqlQueryBuilder {
            query: String::new(),
            bind_idx: 1,
        }
    }
}

impl Default for MssqlQueryBuilder {
    fn default() -> Self {
        MssqlQueryBuilder::new()
    }
}

impl QueryBuilder<Mssql> for MssqlQueryBuilder {
    fn push_sql(&mut self, sql: &str) {
        self.query.push_str(sql);
    }

    fn push_identifier(&mut self, identifier: &str) -> diesel::QueryResult<()> {
        self.push_sql("[");
        self.push_sql(&identifier);
        self.push_sql("]");
        Ok(())
    }

    fn push_bind_param(&mut self) {
        let param = format!("@P{}", self.bind_idx);
        self.bind_idx += 1;
        self.query.push_str(&param);
    }

    fn finish(self) -> String {
        self.query
    }
}

impl<L, O> QueryFragment<Mssql> for LimitOffsetClause<L, O>
where
    L: QueryFragment<Mssql>,
    O: QueryFragment<Mssql>,
{
    fn walk_ast<'b>(
        &'b self,
        mut out: diesel::query_builder::AstPass<'_, 'b, Mssql>,
    ) -> diesel::QueryResult<()> {
        out.push_sql(" TOP ");
        
        self.limit_clause.walk_ast(out.reborrow())?;
        self.offset_clause.walk_ast(out.reborrow())?;
        Ok(())
    }
    
}


// impl<E> QueryFragment<Mssql> for diesel::query_builder::LimitClause<E>
// where
//     diesel::query_builder::LimitClause<E>: QueryFragment<Mssql>,
// {
//     fn walk_ast<'b>(
//         &'b self,
//         pass: diesel::query_builder::AstPass<'_, 'b, Mssql>,
//     ) -> diesel::QueryResult<()> {
//         todo!()
//     }
// }
