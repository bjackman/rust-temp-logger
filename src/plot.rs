extern crate nix;
extern crate tempfile;

use std::time::{ SystemTime, Duration };
use crate::db::TempDb;
use gnuplot::{ Figure };
use std::fs::File;
use std::io::Read;
use nix::unistd;
use nix::sys::stat;
use tempfile::tempdir;
use std::thread;
use std::sync::mpsc;

// Read some data from a temperature database, plot it, and return a plot as PNG data
pub fn plot_png(db: &mut TempDb) -> Vec<u8> {
    let records = db.get_records().expect("Failed to query records");

    let x: Vec<Duration> = records.iter().map(|r| {
        r.time.duration_since(SystemTime::UNIX_EPOCH).unwrap()
    }).collect();

    let y: Vec<f64> = records.iter().map(|r| {
        r.temp.value
    }).collect();

    // Plotting to a file then reading it back isn't reliable; instead of
    // faffing with fsync or something we'll just use a named pipe.
    let tmp_dir = tempdir().expect("Failed to create temp dir for plotting");
    let fifo_path = tmp_dir.path().join("plot.pipe");
    // Permissions are read+write for owner.
    unistd::mkfifo(&fifo_path, stat::Mode::S_IRUSR | stat::Mode::S_IWUSR)
        .expect("Failed to create named pipe for plotting");

    let (tx, rx) = mpsc::channel();
    let fifo_path2 = fifo_path.clone(); // TODO I hate this!
    thread::spawn(move || {
        println!("opening");
        let mut file = File::open(fifo_path2).expect("Failed to open named pipe for plotting");

        let mut plot_png_data = Vec::new();
        println!("Going into read");
        file.read_to_end(&mut plot_png_data).expect("Failed to read PNG data from file");
        println!("done read");

        // TODO get rid of this memcpy?
        tx.send(plot_png_data)
    });

    println!("hello");

    let mut fg = Figure::new();
    fg.set_terminal("png", &fifo_path.to_string_lossy());
    fg.axes2d().lines(&x, &y, &[]);
    println!("calling show");
    fg.show().unwrap();
    fg.close();

    println!("waiting for plot data");
    rx.recv().unwrap()
}
