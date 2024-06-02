
use rusqlite::{Connection, Error};

pub trait Table<T> {
  fn list(conn: &Connection, offset: usize, limit: usize) -> Result<Vec<T>, Error>;
  fn create_table(conn: &Connection) -> Result<usize, Error>;
  fn upsert(conn: &Connection, data: &T) -> Result<usize, Error>;
}

pub fn open() -> Result<Connection, Error> {
  let path = "./db_test.db";
  Connection::open(path)
}
