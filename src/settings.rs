use std::env;
use config::{File, Config, Environment};
use anyhow::{Result, Context};
use serde::Deserialize;
use lazy_static::lazy_static;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub host: String,
    pub port: i32,
    pub database: Database,
}

impl Settings {
    pub fn new() -> Result<Self> {
        let mut s = Config::default();

        s.merge(File::with_name(&format!("settings/default")))
            .context("Unable to load default.json")?;

        let env = env::var("ENV").unwrap_or("development".into());
        s.merge(File::with_name(&format!("settings/{}", env)).required(false))
            .context(format!("Unable to load {}.json", env))?;

        s.merge(Environment::new().separator("_".into()))?;

        s.try_into().context("Unable to parse json into settings struct")
    }
}

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().unwrap();
}
