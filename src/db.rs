use std::time::{ SystemTime, SystemTimeError, UNIX_EPOCH, Duration };
use uom::si::{ f64::ThermodynamicTemperature };
use rusqlite::{ Connection, Statement, params, };
use std::fmt;

pub type Temp = ThermodynamicTemperature;
pub use uom::si::thermodynamic_temperature::{ degree_celsius, kelvin };

#[derive(Debug)]
pub struct TempRecord {
    pub time: SystemTime,
    pub temp: Temp,
}

pub struct TempDb<'a> {
    insert_stmt: Statement<'a>,
    query_stmt: Statement<'a>,
}

#[derive(Debug)]
pub enum Error {
    SqliteError(rusqlite::Error), // Error passed on from SQLite DB
    TimestampError,               // You gave a timestamp that couldn't be stored
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self  {
            Error::SqliteError(err) => {
                write!(f, "SQLite error: {}", err)
            },
            Error::TimestampError => {
                write!(f, "Invalid timstamp")
            }
        }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        return Error::SqliteError(error);
    }
}

impl From<SystemTimeError> for Error {
    fn from(_: SystemTimeError) -> Self {
        return Error::TimestampError;
    }
}

impl std::error::Error for Error {} // Just use defaults

type Result<T> = std::result::Result<T, Error>;

impl<'a> TempDb<'a> {
    pub fn new(conn: &'a Connection) -> Result<TempDb<'a>> {
        conn.execute(
            "CREATE TABLE temperature (
                 id          INTEGER PRIMARY KEY,
                 timestamp_s INTEGER,
                 temp_k      REAL)",
            params![])?;

        let insert_stmt = conn.prepare(
            "INSERT INTO temperature (timestamp_s, temp_k)
                    VALUES ((:timestamp_s), (:temp_k))",)?;
        let query_stmt = conn.prepare(
            "SELECT timestamp_s, temp_k FROM temperature")?;

        Ok(TempDb {
            insert_stmt,
            query_stmt,
        })
    }

    pub fn insert(&mut self, time: SystemTime, temp: Temp) -> Result<()> {
        if let Ok(duration) = time.duration_since(UNIX_EPOCH) {
            let now_s = duration.as_secs();
            // uom stores values in SI base unit, hence accessing .value gets us
            // Kelvin
            // TODO is there a better way of doing this where we don't have to rely
            // on this knowledge?
            self.insert_stmt.execute_named(&[(":timestamp_s", &(now_s as i64)),
                                             (":temp_k", &(temp.value))])?;
            Ok(())
        } else {
            Err(Error::TimestampError)
        }
    }

    #[allow(dead_code)]
    pub fn insert_now(&mut self, temp: Temp) -> Result<()> {
        self.insert(SystemTime::now(), temp)
    }

    // TODO Return an iterator instead of a vector
    pub fn get_records(&mut self) -> Result<Vec<TempRecord>> {
        let results: Vec<TempRecord> = self.query_stmt.query_map(params![], |row| {
            // rusqlite won't directly give us a u64 because SQLite can't
            // store them. Rust won't silently cast an i64 to u64 because it's
            // lossy. So we explicitly .get an i64, then explicitly cast to u64.
            let timestamp_s: i64 = row.get(0)?;
            Ok(TempRecord {
                time: UNIX_EPOCH + Duration::new(timestamp_s as u64, 0),
                temp: Temp::new::<kelvin>(row.get(1)?)
            })
        })?.map(std::result::Result::unwrap).collect();
        Ok(results)
    }
}


#[cfg(test)]
mod tests {
    use std::time::{ SystemTime, Duration };
    use rusqlite::{ Connection };
    use crate::db::{ Temp, degree_celsius, TempDb };

    #[test]
    fn empty() {
        let conn = Connection::open_in_memory().unwrap();
        let mut db = TempDb::new(&conn).unwrap();
        for _ in db.get_records().unwrap() {
            panic!("Found row in DB that should be empty")
        }
    }

    #[test]
    fn timestamp() {
        let conn = Connection::open_in_memory().unwrap();
        let mut db = TempDb::new(&conn).unwrap();

        let time = SystemTime::UNIX_EPOCH.checked_add(Duration::new(1000, 0)).unwrap();
        db.insert(time, Temp::new::<degree_celsius>(50.0)).unwrap();

        let records = db.get_records().unwrap();

        assert_eq!(records.len(), 1);
        assert_eq!(records.get(0).unwrap().time, time);
    }

    #[test]
    fn two_rows() {
        let conn = Connection::open_in_memory().unwrap();
        let mut db = TempDb::new(&conn).unwrap();

        db.insert_now(Temp::new::<degree_celsius>(30.0)).unwrap();
        db.insert_now(Temp::new::<degree_celsius>(-40.0)).unwrap();
        let records = db.get_records().unwrap();

        assert_eq!(records.len(), 2);
        assert_eq!(records.get(0).unwrap().temp, Temp::new::<degree_celsius>(30.0));
        assert_eq!(records.get(1).unwrap().temp, Temp::new::<degree_celsius>(-40.0));
    }
}
