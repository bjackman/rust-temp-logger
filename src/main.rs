mod db;

use db::TempDb;
use std::error::Error;
use rusqlite::Connection;

fn main() -> Result<(), Box<dyn Error>> {
    // TODO this sucks.
    // See the probem described here:
    // https://stackoverflow.com/questions/48117710/return-a-reference-together-with-the-referenced-object-in-rust
    // Having the Conncetion allocated in the caller gets around hte issue, but
    // I'm not happy with it.
    let conn = Connection::open_in_memory()?;
    let mut db = TempDb::new(&conn)?;
    db.insert()?;

    Ok(())
}
