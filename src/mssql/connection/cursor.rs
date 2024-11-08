use super::row::MssqlRow;
use diesel::result::QueryResult;
use tiberius::Row;

pub struct Cursor {
    pub rows: Vec<Row>,
}

impl Cursor {
    pub fn new(rows: Vec<Row>) -> Self {
        Self { rows }
    }
}

impl Iterator for Cursor {
    type Item = QueryResult<MssqlRow>;

    fn next(&mut self) -> Option<Self::Item> {
        //TODO: Use something else than pop()...
        self.rows.pop().map(|r| Ok(MssqlRow { inner_row: r }))
    }
}
