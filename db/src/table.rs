use rusqlite::{Connection, Result};
use std::fmt;

pub struct TableCol {
    pub name: String,
    pub data_type: String,
    pub constraint: Option<Vec<String>>,
}

impl fmt::Display for TableCol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let constraints = self.constraint.as_deref().unwrap_or(&[] as &[String]).join(" ");
        write!(f, "{} {} {}", &self.name, &self.data_type, constraints)
    }
}

//pub trait Table {
//    fn get_cols() -> Vec<TableCol>;
//}

pub struct DbTable<'a> {
    conn: &'a Connection,
    name: String,
    columns: Vec<TableCol>,
}

impl<'a> DbTable<'a> {

    pub fn create(
        name: String,
        columns: Vec<TableCol>,
        conn: &'a Connection,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let table = DbTable{ conn, name, columns };

        let cols:String = table.columns
            .iter()
            .map(|c| format!("{}", c))
            .collect::<Vec<String>>()
            .join(",");
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} ({})",
            &table.name,
            &cols
        );
        table.conn.execute(&sql, ())?;

        Ok(table)
    }

    //pub fn upsert(cols: &str[], values)

}
