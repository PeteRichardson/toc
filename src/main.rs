//use toc::parse_log;
use std::path::PathBuf;
use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs::File;
use log::info;
use regex::Regex;

fn find_lines(path: &PathBuf, re: Regex) -> Result<Vec<String>, std::io::Error> {
    info!("searching '{:?}' for '{}'", path, re);
    let file = BufReader::new(File::open(path)?);
    let matches : Vec<String> = file.lines()
        .map(|i| i.unwrap())
        .filter(|l| re.is_match(l))
        .collect();
    Ok(matches)
}

fn main() -> Result<(), Box<dyn Error>> {

    env_logger::init();

    const FILENAME: &str = "dlog0.log";
    let filepath = PathBuf::from(FILENAME);
    let pattern = Regex::new(r"^[\+]+ (Section [\d\.]+)")?;
    let lines: Vec<String> = find_lines(&filepath, pattern)?;
    for line in lines {
        println!("{}", line);
    }
    Ok(())
}
