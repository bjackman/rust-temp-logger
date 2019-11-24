use std::error::Error;

mod db;

fn main() -> Result<(), Box<dyn Error>> {
    return db::go();
}
