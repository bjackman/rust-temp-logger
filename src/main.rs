mod db;
mod web;
mod sensor;
mod plot;

use std::time::{ SystemTime, Duration };
use db::{ Temp, degree_celsius, TempDb };
use std::error::Error;
use rusqlite::Connection;
use std::sync::{ Arc, Mutex, Condvar };
use std::thread;
use std::thread::sleep;

#[macro_use] extern crate clap;
#[macro_use] extern crate log;
extern crate env_logger;

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

// Poll the sensor until the bool guarded by the Mutex+Condvar is set to true.
// Write temperature recordings into the database as they are made
fn do_poll(mutex_cond: Arc<(Mutex<bool>, Condvar)>, mut db: TempDb) {
    let (mutex, cond) = &*mutex_cond;
    let mut done = mutex.lock().unwrap();
    loop {
        db.insert_now(sensor::poll()).expect("Failed to insert into database");

        let result = cond.wait_timeout(done, Duration::from_millis(1000)).unwrap();
        // Can't do re-structuring assignment without "let" hence use of
        // intermediate variable here
        done = result.0;
        if *done {
            break;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
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
    // Spin up a thread to poll the sensor
    let poll_killer = Arc::new((Mutex::new(false), Condvar::new()));
    let poll_killer2 = poll_killer.clone();
    let db_path2 = db_path.to_owned();
    thread::spawn(move|| {
        let conn_write = Connection::open(db_path2).expect("Error opening DB connection");
        let db_write = TempDb::new(&conn_write).expect("Error connecting to database");

        do_poll(poll_killer2, db_write);
    });

    let conn = Connection::open(db_path)?;
    let mut db = TempDb::new(&conn)?;

    web::serve(&mut db);

    // Gracefully terminate the sensor polling thread
    sleep(Duration::from_millis(15));
    let (lock, cond) = &*poll_killer;
    {
        info!("Terminating polling thread");
        let mut done = lock.lock().unwrap();
        *done = true;
        cond.notify_all();
    }

    info!("Done");
    Ok(())
}
