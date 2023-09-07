mod settings;

use crate::settings::Settings;


fn main() {
    let settings = Settings::new();

    // Print out our settings (as a HashMap)
    println!(
        "{:?}",
        settings
    );
}