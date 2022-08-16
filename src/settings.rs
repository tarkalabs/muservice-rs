use std::env;
use config::{File, Config, Environment, FileFormat};
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
        let env = env::var("ENV").ok();
        let mut builder = Config::builder()
            .add_source(File::new("settings/default", FileFormat::Json))
            .add_source(Environment::default().separator("_"));

        builder = match env {
            Some(_) => builder.add_source(File::new(&format!("settings/{}", env.unwrap()), FileFormat::Json)),
            None => builder,
        };

        let config = builder.build()?;
        config.try_deserialize().context("Failed to parse JSON into Settings struct.")
    }
}

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().unwrap();
}
