use std::collections::HashSet;
use std::path::PathBuf;
use url::Url;

pub trait Reader {
    fn read_line(&mut self) -> String;
}

pub struct StdinReader {}

impl StdinReader {
    pub fn new() -> Self {
        Self {}
    }
}

impl Reader for StdinReader {
    fn read_line(&mut self) -> String {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer
    }
}


pub fn read_until_empty<R: Reader>(reader: &mut R) -> (HashSet<Url>, HashSet<PathBuf>) {
    let mut urls = HashSet::new();
    let mut files = HashSet::new();

    loop {
        println!("Enter a url or a file path (empty line to finish):");
        let line = reader.read_line();
        if line.trim().is_empty() {
            break;
        }
        if let Ok(url) = Url::parse(&line) {
            urls.insert(url);
        } else if PathBuf::from(&line).exists() {
            files.insert(PathBuf::from(&line));
        }
    }

    (urls, files)
}