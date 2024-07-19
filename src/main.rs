mod toc;
use crate::toc::TocEntry;
use clap::Parser;
use log::info;
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    /// file to extract toc from
    input_file: PathBuf,
}

fn extract_toc_entries(lines: Vec<String>, re: Regex) -> Vec<TocEntry> {
    // Search for a regex pattern in a vector of strings
    // return a list of table of contents entries
    info!("searching for '{}'", re);
    let mut toc_entries: Vec<TocEntry> = Vec::new();
    for (offset, line) in lines.iter().enumerate() {
        if re.is_match(line) {
            let entry = TocEntry::new(offset, line.clone());
            toc_entries.push(entry);
        }
    }
    toc_entries
}

fn load_lines(path: &PathBuf) -> Result<Vec<String>, std::io::Error> {
    // Load lines from a file into a vector of strings
    let file = BufReader::new(File::open(path)?);
    let lines: Vec<String> = file.lines().map(|i| i.unwrap()).collect();
    Ok(lines)
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let args = Args::parse();

    let filepath = args.input_file;
    let pattern = Regex::new(r"^[\+]+ (Section [\d\.]+)")?;
    let lines = load_lines(&filepath)?;
    let toc_entries: Vec<TocEntry> = extract_toc_entries(lines, pattern);

    for entry in toc_entries {
        println!("{}", entry);
    }
    Ok(())
}
