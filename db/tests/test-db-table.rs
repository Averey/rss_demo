use rusqlite::Connection;
use db::table::{DbTable, TableCol};

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
    DbTable::new("test", cols).connect(&conn).create_table()?;
    let mut stmt = conn.prepare("SELECT * FROM pragma_table_info('test')")?;
    let _col_names = stmt.query_map([], |row| {
        //Ok(TableCol {
        //    name: row.get(0)?,
        //    data_type:  row.get(1)?,
        //    constraint: None,
        //})
        //Ok(row.get::<usize, String>(0))
        row.get::<usize, String>(0)
        //row.get(0)
    });

    //let row: String = conn.query_row(
    //    "SELECT * FROM pragma_table_info('test')",
    //    [],
    //    |row| row.get(0) 
    //).unwrap();
    //println!("{}", row);
    Ok(())
}
