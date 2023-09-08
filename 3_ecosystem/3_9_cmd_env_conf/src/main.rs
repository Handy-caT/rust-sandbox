mod settings;
use std::path::PathBuf;

use clap::{arg, ArgAction, Command, command, Parser, value_parser};
use crate::settings::Settings;



fn main() {
    let matches = command!()
        .about("Prints its configuration to STDOUT.")
        .override_usage("step_3_9 [FLAGS] [OPTIONS]")
        .arg(
            arg!(
                -c --config <FILE> "Path to configuration file [env: CONF_FILE=] [default: config.toml]"
            )
                // We don't have syntax yet for optional options, so manually calling `required`
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(
            -d --debug "Turn debugging information on"
        ))
        .get_matches();

    let debug = matches.get_flag("debug");
    let filename = matches.get_one::<PathBuf>("config");
    if let Some(filename) = filename {
        let path = filename.file_name().unwrap().to_string_lossy();
        println!("Using config file: {:?}", path);

        let settings = Settings::new(debug, path);

        // Print out our settings (as a HashMap)
        println!(
            "{:?}",
            settings
        );
    } else {
        let settings = Settings::new(debug, "config");

        // Print out our settings (as a HashMap)
        println!(
            "{:?}",
            settings
        );
    }




}