use log::info;
use regex::Regex;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

// A table of contents entry is a tuple of line offset and the line itself
struct TocEntry {
    offset: usize,
    line: String,
}

impl TocEntry {
    fn new(offset: usize, line: String) -> TocEntry {
        TocEntry { offset, line }
    }
}

impl fmt::Display for TocEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line offset {:3?} - {}", self.offset, self.line)
    }
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
    // Load a file, search for a regex pattern, generate a list of
    // table of contents entries and print them
    let filepath = PathBuf::from("dlog0.log"); // TODO: fix hard-coded path

    let pattern = Regex::new(r"^[\+]+ (Section [\d\.]+)")?;
    let lines = load_lines(&filepath)?;
    let toc_entries: Vec<TocEntry> = extract_toc_entries(lines, pattern);

    for entry in toc_entries {
        println!("{}", entry);
    }
    Ok(())
}
