mod settings;

use config::{Config, Environment, File};
use config::ValueKind::String;
use crate::settings::Settings;


fn main() {
    let settings = Settings::new();

    // Print out our settings (as a HashMap)
    println!(
        "{:?}",
        settings
    );
}