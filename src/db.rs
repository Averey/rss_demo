use rusqlite::{Connection, Result};

use crate::models::*;

pub fn init() -> Result<()>{
  let conn = Connection::open_in_memory()?;
  Channel::creat_table(&conn)?;
  Ok(())
}

pub trait Table {
  fn save() -> Result<()> {
    Ok(())
  }
    
}

pub struct Db {
  conn: Connection
}

impl Db {
  fn new() -> Result<Self> {
    let conn = Connection::open_in_memory()?;
    Ok(
      Self {
        conn,
      }
    )
  }
    
}