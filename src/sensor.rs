use crate::db::{ Temp, degree_celsius };
use rand::random;
use std::thread::sleep;
use std::time::Duration;

// Global state used to make the randomly generated values look sort of nice
static mut LAST_TEMP_C: f64 = 10.0;

/// Poll the temperature sensor. Note this could take some time
/// This is actually just a stub
pub fn poll() -> Temp {
    sleep(Duration::from_millis(random::<u8>().into()));
    let temp_delta = (random::<f64>() % 1.0) - 2.0;
    let temp_c;
    unsafe { // To allow accessing mutable static
        temp_c = LAST_TEMP_C + temp_delta;
        LAST_TEMP_C = temp_c;
    }
    Temp::new::<degree_celsius>(temp_c)
}
