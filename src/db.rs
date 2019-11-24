use std::time::{ SystemTime, UNIX_EPOCH, Duration };
use std::error::Error;
use uom::si::{ f64::ThermodynamicTemperature };
use rusqlite::{ Connection, Statement, params };

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

impl<'a> TempDb<'a> {
    pub fn new(conn: &'a Connection) -> Result<TempDb<'a>, Box<dyn Error>> {
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

    pub fn insert(&mut self, temp: Temp) -> Result<(), Box<dyn Error>> {
        let now_s = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        // uom stores values in SI base unit, hence accessing .value gets us
        // Kelvin
        // TODO is there a better way of doing this where we don't have to rely
        // on this knowledge?
        self.insert_stmt.execute_named(&[(":timestamp_s", &(now_s as i64)),
                                         (":temp_k", &(temp.value))])?;
        Ok(())
    }

    // TODO Return an iterator instead of a vector
    pub fn get_records(&mut self) -> Result<Vec<TempRecord>, Box<dyn Error>> {
        let results: Vec<TempRecord> = self.query_stmt.query_map(params![], |row| {
            let timestamp_s: i64 = row.get(0)?; // s64?
            Ok(TempRecord {
                time: UNIX_EPOCH + Duration::new(timestamp_s as u64, 0),
                temp: Temp::new::<kelvin>(row.get(1)?)
            })
        })?.map(Result::unwrap).collect();
        Ok(results)
    }
}


#[cfg(test)]
mod tests {
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
    fn two_rows() {
        let conn = Connection::open_in_memory().unwrap();
        let mut db = TempDb::new(&conn).unwrap();

        db.insert(Temp::new::<degree_celsius>(30.0)).unwrap();
        db.insert(Temp::new::<degree_celsius>(-40.0)).unwrap();
        let records = db.get_records().unwrap();

        assert_eq!(records.len(), 2);
        assert_eq!(records.get(0).unwrap().temp, Temp::new::<degree_celsius>(30.0));
        assert_eq!(records.get(1).unwrap().temp, Temp::new::<degree_celsius>(-40.0));
    }
}
