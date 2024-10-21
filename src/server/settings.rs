#![allow(dead_code)]

use anyhow::Result;
use clap::{CommandFactory, Parser};
use twelf::{config, Layer};

#[config]
#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct Settings {
    #[clap(long, help = "The host to bind to", default_value = "127.0.0.1")]
    pub host: String,
    #[clap(long, short, help = "The port to bind to", default_value_t = 3000)]
    pub port: i32,
}

pub fn load_settings() -> Result<Settings> {
    let config = Settings::with_layers(&[
        Layer::Env(Some("DEMO_".to_string())),
        Layer::Clap(Settings::command().get_matches()),
    ])?;

    Ok(config)
}
