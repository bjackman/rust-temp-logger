use std::time::{ SystemTime, UNIX_EPOCH, Duration };
use std::error::Error;
use uom::si::{ f64::ThermodynamicTemperature, thermodynamic_temperature::degree_celsius };
use rusqlite::{ Connection, Statement, params };

#[derive(Debug)]
pub struct TempRecord {
    pub time: SystemTime,
    pub temp: ThermodynamicTemperature,
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
                 temp_c      REAL)",
            params![])?;

        let insert_stmt = conn.prepare(
            "INSERT INTO temperature (timestamp_s, temp_c)
                    VALUES ((:timestamp_s), (:temp_c))",)?;
        let query_stmt = conn.prepare(
            "SELECT timestamp_s, temp_c FROM temperature")?;

        Ok(TempDb {
            insert_stmt,
            query_stmt,
        })
    }

    pub fn insert(&mut self) -> Result<(), Box<dyn Error>> {
        let now_s = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        self.insert_stmt.execute_named(&[(":timestamp_s", &(now_s as i64)),
                                         (":temp_c", &30)])?;
        Ok(())
    }

    // TODO Return an iterator instead of a vector
    pub fn get_records(&mut self) -> Result<Vec<TempRecord>, Box<dyn Error>> {
        let results: Vec<TempRecord> = self.query_stmt.query_map(params![], |row| {
            let timestamp_s: i64 = row.get(0)?; // s64?
            Ok(TempRecord {
                time: UNIX_EPOCH + Duration::new(timestamp_s as u64, 0),
                temp: ThermodynamicTemperature::new::<degree_celsius>(row.get(1)?)
            })
        })?.map(Result::unwrap).collect();
        Ok(results)
    }
}

