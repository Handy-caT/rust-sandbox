use std::collections::HashSet;
use std::hash::Hash;
use std::path::PathBuf;
use url::Url;

struct FileParser {
    file: PathBuf
}

impl FileParser {
    fn new(file: PathBuf) -> Self {
        Self {
            file
        }
    }

    async fn parse_file(&self) -> (HashSet<Url>, HashSet<PathBuf>) {
        let mut urls = HashSet::new();
        let mut files = HashSet::new();

        let contents = tokio::fs::read_to_string(&self.file).await.unwrap();

        for line in contents.lines() {
            if let Ok(url) = Url::parse(line) {
                urls.insert(url);
            } else if PathBuf::from(line).exists() {
                files.insert(PathBuf::from(line));
            }
        }

        (urls, files)
    }
}
