use std::time::{ SystemTime, UNIX_EPOCH, Duration };
use std::error::Error;
use uom::si::{ f64::ThermodynamicTemperature, thermodynamic_temperature::degree_celsius };
use rusqlite::{ Connection, params, };

#[derive(Debug)]
pub struct TempRecord {
    pub time: SystemTime,
    pub temp: ThermodynamicTemperature,
}

pub struct TempDb {
    conn: Connection,
}

impl TempDb {
    pub fn new() -> Result<TempDb, Box<dyn Error>> {
        let conn = Connection::open_in_memory()?;

        conn.execute(
            "CREATE TABLE temperature (
                 id          INTEGER PRIMARY KEY,
                 timestamp_s INTEGER,
                 temp_c      REAL)",
            params![])?;

        Ok(TempDb {
            conn
        })
    }

    pub fn insert(&mut self) -> Result<(), Box<dyn Error>> {
        let now_s = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        // TODO use prepared satetment
        self.conn.execute(
            "INSERT INTO temperature (timestamp_s, temp_c)
                 VALUES (?1, ?2)",
            // TODO how to get temp as int?
            params![now_s as i64, 30i32])?;
        Ok(())
    }

    // TODO hide MappedRows from caller?
    pub fn iter(&self) -> Result<(), Box<dyn Error>> {
        // TODO use prepared satetment
        let mut statement = self.conn.prepare(
            "SELECT timestamp_s, temp_c FROM temperature")?;
        // TODO figure out how to return an iterator
        let row_iter = statement.query_map(params![], |row| {
            let timestamp_s: i64 = row.get(0)?; // s64?
            Ok(TempRecord {
                time: UNIX_EPOCH + Duration::new(timestamp_s as u64, 0),
                temp: ThermodynamicTemperature::new::<degree_celsius>(row.get(1)?)
            })
        })?;
        for record in row_iter {
            println!("{:?}", record.unwrap());
        }
        Ok(())
    }
}

