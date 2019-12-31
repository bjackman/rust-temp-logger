extern crate tempfile;

use std::time::SystemTime;
use crate::db::TempDb;
use gnuplot::{ Figure, AxesCommon };
use std::fs::File;
use std::io::Read;
use tempfile::tempdir;

// Read some data from a temperature database, plot it, and return a plot as PNG data
pub fn plot_png(db: &mut TempDb) -> Vec<u8> {
    let records = db.get_records().expect("Failed to query records");

    let x: Vec<u64> = records.iter().map(|r| {
        r.time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
    }).collect();

    let y: Vec<f64> = records.iter().map(|r| {
        r.temp.value
    }).collect();

    let tmp_dir = tempdir().expect("Failed to create temp dir for plotting");
    let png_path = tmp_dir.path().join("plot.png");
    let png_path = png_path.to_str().unwrap();
    let mut fg = Figure::new();
    fg.set_terminal("png", png_path);
    fg.axes2d()
        .set_y_label("Temp (Kelvin)", &[])
        .lines(&x, &y, &[]);
    fg.show().expect("Gnuplot failed");
    fg.close();

    let mut file = File::open(png_path).expect("Failed to open plot file");
    let mut plot_png_data = Vec::new();
    file.read_to_end(&mut plot_png_data).expect("Failed to read PNG data from file");

    plot_png_data
}
