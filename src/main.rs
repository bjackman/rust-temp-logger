mod db;
mod web;

use std::time::{ SystemTime, Duration };
use db::{ Temp, degree_celsius, TempDb };
use std::error::Error;
use rusqlite::Connection;
use gnuplot::{ Figure };

fn main() -> Result<(), Box<dyn Error>> {
    // TODO this sucks.
    // See the probem described here:
    // https://stackoverflow.com/questions/48117710/return-a-reference-together-with-the-referenced-object-in-rust
    // Having the Connection allocated in the caller gets around the issue, but
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
