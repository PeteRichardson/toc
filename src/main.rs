//use toc::parse_log;
use std::path::Path;
use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs::File;
use log::info;

fn count_lines(path: &Path) -> Result<usize, std::io::Error> {
    info!("counting '{:?}'", path);
    let mut lines = BufReader::new(File::open(path)?).lines();
    let count = lines.try_fold(0, |acc, line| line.map(|_| acc + 1))?;
    Ok(count)
}


fn main() -> Result<(), Box<dyn Error>> {

    env_logger::init();

    const FILENAME: &str = "dlog0.log";
    let filepath = Path::new(FILENAME);
    let count: usize = count_lines(filepath).expect("Couldn't count lines!");
    println!("{} - {} lines", FILENAME, count);

    Ok(())
}
