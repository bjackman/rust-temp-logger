mod db;

use std::time::{ SystemTime, Duration };
use db::{ Temp, degree_celsius, TempDb, TempRecord };
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

    for &(time_s, temp_c) in [(0, 10),
                              (10, 20),
                              (20, 30),
                              (40, 50)].iter() {
        db.insert(SystemTime::UNIX_EPOCH.checked_add(Duration::new(time_s, 0)).unwrap(),
                  Temp::new::<degree_celsius>(temp_c as f64))
            .expect("DB Insert failed");
    }

    for &TempRecord{time, temp} in db.get_records()
            .expect("Failed to query records")
            .iter() {
        println!("{:?}: {:?}", time, temp);
    }

    Ok(())
}
