use rusqlite::Connection;
//mod db::table;
use db::table::{DbTable, TableCol};

#[test]
fn create() -> Result<(), &'static str> {
    let conn = match Connection::open_in_memory() {
        Ok(c) => c,
        Err(..) => return Err(""),
    };

    let cols = vec![TableCol {
        name: "id".to_string(),
        data_type: "INTEGER".to_string(),
        constraint: Some(vec!["PRIMARY KEY".to_string()]),
    }];

    let mut table = DbTable::new("test".to_string(), cols);
    table.connect(&conn);
    //table.create_table();
    match table.create_table() {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    Ok(())
}
