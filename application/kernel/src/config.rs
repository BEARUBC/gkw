use std::env::set_var;

use anyhow::Result;
use config::File;
use serde::Deserialize;

#[cfg(feature = "simulation")]
const CONFIG_PATH: &str = "config/simulation.yaml";

#[cfg(not(feature = "simulation"))]
const CONFIG_PATH: &str = "config/release.yaml";

const RUST_LOG_KEY: &str = "RUST_LOG";

#[cfg(feature = "simulation")]
const RUST_LOG_VALUE: &str = "debug";

#[cfg(not(feature = "simulation"))]
const RUST_LOG_VALUE: &str = "error";

#[derive(Deserialize)]
pub struct Config {
    #[cfg(feature = "simulation")]
    pub components: Components,
}

#[derive(Deserialize)]
pub struct Components {
    #[cfg(feature = "simulation")]
    pub bms: Bms,

    #[cfg(feature = "simulation")]
    pub emg: Emg,
}

#[derive(Deserialize)]
pub struct Bms {
    #[cfg(feature = "simulation")]
    pub host: String,

    #[cfg(feature = "simulation")]
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Emg {
    #[cfg(feature = "simulation")]
    pub host: String,

    #[cfg(feature = "simulation")]
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
