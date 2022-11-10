use std::env::set_var;

use anyhow::Result;
use config::File;
use serde::Deserialize;

#[cfg(feature = "tcp_data")]
const CONFIG_PATH: &str = "config/simulation.yaml";

#[cfg(not(feature = "tcp_data"))]
const CONFIG_PATH: &str = "config/release.yaml";

const RUST_LOG_KEY: &str = "RUST_LOG";

#[cfg(feature = "tcp_data")]
const RUST_LOG_VALUE: &str = "debug";

#[cfg(not(feature = "tcp_data"))]
const RUST_LOG_VALUE: &str = "error";

#[derive(Deserialize)]
pub struct Config {
    #[cfg(feature = "tcp_data")]
    pub components: Components,
}

#[derive(Deserialize)]
pub struct Components {
    #[cfg(feature = "tcp_data")]
    pub bms: Bms,

    #[cfg(feature = "tcp_data")]
    pub emg: Emg,
}

#[derive(Deserialize)]
pub struct Bms {
    #[cfg(feature = "tcp_data")]
    pub host: String,

    #[cfg(feature = "tcp_data")]
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Emg {
    #[cfg(feature = "tcp_data")]
    pub host: String,

    #[cfg(feature = "tcp_data")]
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
