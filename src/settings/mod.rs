use std::{env, time::Duration};

use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: Option<bool>,
    pub serve: Serve,
    // pub database: Database,
    pub otlp: Option<Otlp>,
}

#[derive(Debug, Deserialize)]
pub struct Serve {
    host: String,
    port: u16,
}

impl Serve {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Otlp {
    pub endpoint: String,
    #[serde(with = "humantime_serde")]
    pub interval: Option<Duration>,
    pub token: Option<String>,
    pub organization: Option<String>,
    pub stream: Option<String>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());

        let s = Config::builder()
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name(&format!("config/{run_mode}")).required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::default().separator("_"))
            .build()?;

        s.try_deserialize()
    }
}
