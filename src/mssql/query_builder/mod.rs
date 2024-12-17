mod limit_offset;
mod select;
mod concat;
use super::backend::Mssql;
use diesel::query_builder::QueryBuilder;

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
        self.push_sql(identifier);
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

