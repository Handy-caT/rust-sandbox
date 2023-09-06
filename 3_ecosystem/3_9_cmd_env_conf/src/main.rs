mod settings;

use config::{Config, Environment, File};


fn main() {
    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(File::with_name("config"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(Environment::with_prefix("CONF"))
        .build()
        .unwrap();

    // Print out our settings (as a HashMap)
    println!(
        "{:?}",
        settings
    );
}