use std::env::set_var;

use anyhow::Result;
use config::File;
#[cfg(feature = "simulation")]
use serde::Deserialize;

#[cfg(feature = "simulation")]
const CONFIG_PATH: &'static str = "config/simulation.yaml";

#[cfg(not(feature = "simulation"))]
const CONFIG_PATH: &'static str = "config/main.yaml";

const RUST_LOG_KEY: &'static str = "RUST_LOG";

#[cfg(feature = "simulation")]
const RUST_LOG_VALUE: &'static str = "info";

#[cfg(not(feature = "simulation"))]
pub type Config = ();

#[cfg(feature = "simulation")]
#[derive(Deserialize)]
pub struct Config {
    pub components: Components,
}

#[cfg(feature = "simulation")]
#[derive(Deserialize)]
pub struct Components {
    pub emg: Emg,
    pub bms: Bms,
}

#[cfg(feature = "simulation")]
#[derive(Deserialize)]
pub struct Emg {
    pub host: String,
    pub port: u16,
}

#[cfg(feature = "simulation")]
#[derive(Deserialize)]
pub struct Bms {
    pub host: String,
    pub port: u16,
}

pub fn init() -> Result<Config> {
    set_var(RUST_LOG_KEY, RUST_LOG_VALUE);
    pretty_env_logger::try_init()?;
    let file = File::with_name(CONFIG_PATH);
    let config = config::Config::builder()
        .add_source(file)
        .build()?
        .try_deserialize::<Config>()?;
    Ok(config)
}
