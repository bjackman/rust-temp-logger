mod db;
mod web;
mod sensor;

use std::time::{ SystemTime, Duration };
use db::{ Temp, degree_celsius, TempDb };
use std::error::Error;
use rusqlite::Connection;
use gnuplot::{ Figure };

#[macro_use]
extern crate clap;

#[allow(dead_code)]
fn insert_fake_data(db: &mut TempDb) {
    // Insert some fake data
    for &(time_s, temp_c) in [(0, 10),
                              (10, 20),
                              (20, 30),
                              (40, 50)].iter() {
        db.insert(SystemTime::UNIX_EPOCH.checked_add(Duration::new(time_s, 0)).unwrap(),
                  Temp::new::<degree_celsius>(temp_c as f64))
            .expect("DB Insert failed");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Start by parsing arguments
    let matches = clap_app!(app =>
        (@arg DB_PATH: --db +takes_value "Path to temperature DB. Created if nonexistent")
    ).get_matches();
    let db_path = matches.value_of("db").unwrap_or("/tmp/temp-logger.sqlite");

    // TODO this sucks.
    // See the probem described here:
    // https://stackoverflow.com/questions/48117710/return-a-reference-together-with-the-referenced-object-in-rust
    // Having the Connection allocated in the caller gets around the issue, but
    // I'm not happy with it.
    let conn = Connection::open(db_path)?;
    let mut db = TempDb::new(&conn)?;

    let records = db.get_records().expect("Failed to query records");

    let x: Vec<u64> = records.iter().map(|r| {
        r.time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
    }).collect();

    let y: Vec<f64> = records.iter().map(|r| {
        r.temp.value
    }).collect();

    let mut fg = Figure::new();
    fg.set_terminal("pngcairo", "/tmp/plot.png");
    fg.axes2d()
        .lines(&x, &y, &[]);
    fg.show().unwrap();

    web::serve();

    Ok(())
}
