use std::borrow::Cow;
use clap::Parser;

const DEFAULT_CONF: &str = "/etc/app/app.conf";

#[derive(Parser)]
pub struct Cli {
    #[clap(long)]
    conf: Option<String>,
}


fn main() {
    let args = Cli::parse();
    let mut result = Cow::Borrowed(DEFAULT_CONF);
    match args.conf {
        Some(conf) => {
            result = Cow::Owned(conf);
        }
        None => {
            let env_conf = std::env::var("APP_CONF");
            if let Ok(conf) = env_conf {
                result = Cow::Owned(conf);
            }
        }
    }
    println!("Using conf: {}", result);
}
