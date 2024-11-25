use super::row::MssqlRow;
use diesel::result::QueryResult;
use std::collections::VecDeque;
use tiberius::Row;

pub struct Cursor {
    pub rows: VecDeque<Row>,
}

impl Cursor {
    pub fn new(rows: VecDeque<Row>) -> Self {
        Self { rows }
    }
}

impl Iterator for Cursor {
    type Item = QueryResult<MssqlRow>;

    fn next(&mut self) -> Option<Self::Item> {
        self.rows.pop_front().map(|r| Ok(MssqlRow { inner_row: r }))
    }
}
