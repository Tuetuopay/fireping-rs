use rusqlite::{Connection, Result, NO_PARAMS};
use crate::Target;

pub struct Db {
    db: Connection
}

impl Db {
    pub fn new() -> Result<Db> {
        Ok(Db { db: Connection::open("fireping.db")? })
    }

    pub fn init(&self) -> Result<usize> {
        self.db.execute(
            "create table if not exists targets (
                id integer primary key,
                target text not null unique,
                name text not null
            )",
            NO_PARAMS
        )
    }

    pub fn targets(&self) -> Result<Vec<Target>> {
        let sql = "select targets.target, targets.name from targets";
        let mut query = self.db.prepare(sql)?;

        let ips: Vec<Target> =
            query.query_map(NO_PARAMS, |row| Target {
                target: row.get(0), name: row.get(1)
            })?
            .filter_map(|t| match t { Ok(t) => Some(t), _ => None })
            .collect();

        Ok(ips)
    }
}
