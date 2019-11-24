mod db;

use db::TempDb;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut db = TempDb::new()?;
    db.insert()?;
    db.iter()?;

    Ok(())
}
