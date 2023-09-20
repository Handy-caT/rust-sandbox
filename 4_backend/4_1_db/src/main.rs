use clap::Parser;
use sea_orm::*;
use crate::cli::cli;
use crate::cli_processor::CliProcessor;
use crate::db_processor::DBProcessor;

mod db;
pub mod cli;
mod cli_processor;
mod db_processor;


fn main() {
    let args = cli().get_matches();
    let processor = CliProcessor::new(args);
    let command = processor.process_cli();

    let default_parallelism_approx = 12;

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(default_parallelism_approx)
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let db = db::db_connect().await;
        let db_processor = DBProcessor::new(db);

        db_processor.process_command(command).await;
    });
}
