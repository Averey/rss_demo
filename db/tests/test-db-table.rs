use db::table::{DbTable, TableCol};
use rusqlite::Connection;
use serde_json::value;

#[test]
fn create() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open_in_memory()?;
    let cols = vec![
        TableCol {
            name: "id".to_string(),
            data_type: "INTEGER".to_string(),
            constraint: Some(vec!["PRIMARY KEY".to_string()]),
        },
        TableCol {
            name: "name".to_string(),
            data_type: "TEXT".to_string(),
            constraint: None,
        },
    ];

    DbTable::create("test".to_string(), cols, &conn)?;
    let mut stmt = conn.prepare("SELECT * FROM pragma_table_info('test')")?;
    //let mut stmt = conn.prepare("SELECT name FROM test")?;
    let rows = stmt.query_map([], |row| row.get::<usize, String>(1))?;

    let mut cols = Vec::new();
    for col in rows {
        cols.push(col?)
    }
    println!("{:?}", cols);

    //let colsname = stmt.column_names();
    //println!("{:?}", colsname);

    Ok(())
}
