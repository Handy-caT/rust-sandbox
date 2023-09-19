use crate::cli::cli;
use crate::cli_processor::CliProcessor;

mod cli;
mod cli_processor;
mod web_processor;
pub mod requests;

fn main() {
    let args = cli().get_matches();
    let processor = CliProcessor::new(args);
    let command = processor.process_cli();

    let web_processor = web_processor::WebProcessor::new("http://127.0.0.1:8080".parse().unwrap());

    let default_parallelism_approx = 1;

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(default_parallelism_approx)
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let res = web_processor.process_command(command).await;
        println!("{}", res);
    });
}

