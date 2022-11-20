use std::{ops::Deref, path::Path};

use abi::Config;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub struct TestConfig {
    pub config: Config,
}

impl Deref for TestConfig {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl TestConfig {
    pub fn new(filename: impl AsRef<Path>) -> Self {
        let config = Config::load(filename).unwrap();

        Self { config }
    }

    pub fn with_server_port(port: u16) -> Self {
        let mut config = TestConfig::default();
        config.config.server.port = port;
        config
    }
}

impl Default for TestConfig {
    fn default() -> Self {
        Self::new("fixtures/config.toml")
    }
}

#[allow(unused)]
pub fn rand_str() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}
