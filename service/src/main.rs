use std::{env, path::Path};

use ::service::start_server;
use abi::Config;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let filename = env::var("MONGODB_TRIGGER_CONFIG").unwrap_or_else(|_| {
        let p1 = Path::new("./mongodb-trigger.toml");
        let path = shellexpand::tilde("~/.config/mongodb-trigger.toml");
        let p2 = Path::new(path.as_ref());
        let p3 = Path::new("/etc/mongodb-trigger.toml");

        match (p1.exists(), p2.exists(), p3.exists()) {
            (true, _, _) => p1.to_str().unwrap().to_string(),
            (_, true, _) => p2.to_str().unwrap().to_string(),
            (_, _, true) => p3.to_str().unwrap().to_string(),
            _ => panic!("no config file found"),
        }
    });

    tracing_subscriber::fmt::init();

    let config = Config::load(filename)?;
    start_server(&config).await
}
