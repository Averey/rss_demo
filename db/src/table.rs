use rusqlite::{Connection, Result};

pub struct TableCol {
    name: String,
    data_type: String,
    constraint: Option<Vec<String>>,
}

//pub struct TableInfo {
//    name: String,
//    cols: Vec<TableCol>,
//}

pub struct DbTable<'a> {
    conn: Option<&'a Connection>,
    name: String,
    cols: Vec<TableCol>,
}

impl<'a> DbTable<'a> {
    pub fn connect(&mut self, conn: &'a Connection) -> &Self {
        self.conn = Some(conn);
        self
    }

    pub fn new(name: String, cols: Vec<TableCol>) -> Self {
        Self {
            name,
            cols,
            conn: None,
        }
    }

    pub fn create_table(&self) -> Result<(), &'static str> {
        let mut cols_str = String::new();
        self.cols.iter().for_each(|c| {
            let mut constraints = String::new();
            if let Some(ctns) = &c.constraint {
                constraints = ctns.join(" ");
            } else {
            }
            let col = format!("{} {} {}", c.name, c.data_type, constraints);
            cols_str.push_str(&col);
        });

        let sql = format!("CREATE TABLE IF NOT EXISTS {} ({})", &self.name, cols_str);
        match &self.conn {
            Some(ref conn) => {
                if conn.execute(&sql, ()).is_ok() {
                    return Ok(());
                } else {
                    return Err("");
                }
            }
            None => return Err(""),
        };
    }
}

//pub trait DbTable {
//    fn new() -> Self;
//    fn connnet(&self, conn: &Connection) -> Self;
//
//    fn get_conn(&self) -> Connection;
//    fn get_table_info(&self) -> TableInfo;
//
//    fn create_table(&self) -> Result<usize, rusqlite::Error> {
//        let table_info = self.get_table_info();
//        let mut cols = String::new();
//        table_info.cols.iter().for_each(|c| {
//            let mut constraints = String::new();
//            if let Some(ctns) = &c.constraint {
//                constraints = ctns.join(" ");
//            } else {
//            };
//            let col = format!("{} {} {}", c.name, c.data_type, constraints);
//            cols.push_str(&col);
//        });
//        let sql = format!("CREATE TABLE IF NOT EXIsTS {} ({})", table_info.name, cols);
//        self.get_conn().execute(&sql, ())
//    }
//}
