extern crate tempfile;

// use std::time::SystemTime;
use crate::db::TempDb;
use std::time::SystemTime;
use std::io::Write;
use std::process::{ Command, Stdio };
use std::fmt;

#[derive(Debug)]
pub enum Error {
    GnuplotError(std::process::Output), // Something went wrong with Gnuplot
    CommandError(std::io::Error), // Failed to run Gnuplot
}
use Error::{ CommandError, GnuplotError };

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        CommandError(error)
    }
}

// TODO: these errors don't actually get printed by default???
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self  {
            Error::GnuplotError(output) => {
                write!(f, "Gnuplot error: {}", String::from_utf8_lossy(&output.stderr))
            },
            Error::CommandError(err) => {
                write!(f, "Failed to run gnuplot command: {}", err)
            }
        }
    }
}

impl std::error::Error for Error {} // Just use defaults

type Result<T> = std::result::Result<T, Error>;

// Read some data from a temperature database, plot it, and return a plot as PNG data
pub fn plot_png(db: &mut TempDb) -> Result<Vec<u8>> {
    // Note Rust's API doesn't support writing a string directly to stdin in
    // this expression hence the ridiculous dance with spawn() and piping
    let mut gnuplot = Command::new("gnuplot")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    {
        let gnuplot_cmds = include_bytes!("commands.gnuplot");
        let child_stdin = gnuplot.stdin.as_mut().unwrap();
        child_stdin.write_all(gnuplot_cmds)?;
        for r in db.get_records().expect("Failed to query records").iter() {
            let time_s = r.time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
            let temp_k = r.temp.value;
            child_stdin.write(format!("{} {}\n", time_s, temp_k).as_bytes())?;
        }

    }

    let result = gnuplot.wait_with_output()?;
    if result.status.success() {
        Ok(result.stdout)
    } else {
        Err(GnuplotError(result))
    }
}
