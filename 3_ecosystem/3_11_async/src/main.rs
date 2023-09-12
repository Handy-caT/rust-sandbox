use std::path::PathBuf;
use std::thread::{available_parallelism};
use clap::Parser;
use tokio::io::AsyncWriteExt;
use tokio::time::Instant;

const HTML_FILE: &str = "html";

fn get_filename_from_link<S: AsRef<str>>(link: &S) -> String {
    let link = link.as_ref();
    let last_part = link.split("//").last().unwrap();
    let mut last_part = last_part.replace('/', "_");
    last_part = last_part.replace('.', "_");

    last_part
}

async fn download_by_link<L: AsRef<str>, O: AsRef<str>>(link: L, output_dir: &O) {
        let link = link.as_ref();
        println!("link: {}", link);
        let filename = get_filename_from_link(&link);
        let output_dir = output_dir.as_ref();

        let body = reqwest::get(link)
            .await.expect("request failed")
            .text()
            .await.expect("failed to get body");

        let mut file = tokio::fs::File::create(format!("{}/{}.{}", output_dir, filename, HTML_FILE))
            .await
            .expect("failed to create file");

        file.write_all(body.as_bytes()).await.expect("failed to write file");
}

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value_t = available_parallelism().unwrap().get())]
    max_threads: usize,
    file: PathBuf,
}

fn main() {
    let args = Cli::parse();

    let links = std::fs::read_to_string(args.file).unwrap()
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();


    let default_parallelism_approx = args.max_threads;


    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(default_parallelism_approx)
        .enable_all()
        .build()
        .unwrap();

    let output_dir = "output";

    std::fs::create_dir_all(output_dir).unwrap();

    let start = Instant::now();
    let tasks = links
        .into_iter()
        .map(|link| async {
            download_by_link(link, &output_dir).await;
            println!("time: {:?}", start.elapsed());
        })
        .collect::<Vec<_>>();

    runtime.block_on(async {
        futures::future::join_all(tasks).await;
    });

}



