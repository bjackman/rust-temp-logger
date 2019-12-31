use std::time::{ SystemTime, Duration };
use crate::db::TempDb;
use gnuplot::{ Figure };
use std::fs::File;
use std::io::Read;

// Read some data from a temperature database, plot it, and return a plot as PNG data
pub fn plot_png(db: &mut TempDb) -> Vec<u8> {
    let records = db.get_records().expect("Failed to query records");

    let x: Vec<Duration> = records.iter().map(|r| {
        r.time.duration_since(SystemTime::UNIX_EPOCH).unwrap()
    }).collect();

    let y: Vec<f64> = records.iter().map(|r| {
        r.temp.value
    }).collect();

    let mut fg = Figure::new();
    fg.set_terminal("png", "/tmp/plot.png");
    fg.axes2d().lines(&x, &y, &[]);
    fg.show().expect("Gnuplot failed");
    fg.close();

    let mut file = File::open("/tmp/plot.png").expect("Failed to open plot file");
    let mut plot_png_data = Vec::new();
    file.read_to_end(&mut plot_png_data).expect("Failed to read PNG data from file");

    plot_png_data
}
