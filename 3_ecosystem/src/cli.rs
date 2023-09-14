use clap::{arg, Arg, ArgAction, ArgMatches, command, value_parser};
use std::path::PathBuf;
use std::thread::available_parallelism;
use url::Url;

pub struct CLI {}

impl CLI {
    pub fn parse_args() -> ArgMatches {
        let default_parallelism_approx = available_parallelism().unwrap().get();
        command!()
            .about("Prints its configuration to STDOUT.")
            .override_usage("step_3_9 [FLAGS] [OPTIONS]")
            .arg(arg!(
            -d --debug "Turn debugging information on"
            ))
            .arg(
                Arg::new("max_threads")
                    .long("max-threads")
                    .help(format!("Maximum number of threads to use [default: {}]", default_parallelism_approx))
                    .action(ArgAction::Set)
                    .num_args(1)
            )
            .arg(arg!(
                -o --output <FILE> "Path to output dir [env: OUT_DIR=] [default: output]"
            ))
            .arg(arg!(
                --min_quality <QUALITY> "Minimal quality of the output image [default: 70]"
            ))
            .arg(arg!(
                --target_quality <QUALITY> "Target quality of the output image. Use 100 for no color loss [default: 99]"
            ))
            .arg(
                Arg::new("urls")
                    .long("urls")
                    .help("List of urls to process")
                    .value_parser(value_parser!(Url))
                    .conflicts_with("file")
                    .action(ArgAction::Append)
                    .num_args(1..)
            )
            .arg(
                Arg::new("files")
                    .long("files")
                    .help("List of files to process")
                    .value_parser(value_parser!(std::path::PathBuf))
                    .conflicts_with("file")
                    .action(ArgAction::Append)
                    .num_args(1..)
            )
            .arg(Arg::new("file")
                .long("file")
                .help("File with urls and files to process")
                .value_parser(value_parser!(std::path::PathBuf))
                .conflicts_with_all(["urls", "files"])
                .action(ArgAction::Set)
                .num_args(1)
            )
            .arg(
                Arg::new("interactive")
                    .long("interactive")
                    .short('i')
                    .help("Run in interactive mode")
                    .conflicts_with_all(["urls", "files", "file"])
                    .action(ArgAction::SetTrue)
            )
            .arg(
                Arg::new("config")
                    .long("config")
                    .short('c')
                    .help("Path to configuration file")
                    .value_parser(value_parser!(std::path::PathBuf))
                    .action(ArgAction::Set)
                    .num_args(1)
            )
            .get_matches()
    }
}