use std::env::set_var;
use std::env::var;

use anyhow::Result;
use config::File;
use serde::Deserialize;

const CONFIG_PATH_KEY: &str = "CONFIG";

const RUST_LOG_KEY: &str = "RUST_LOG";

#[cfg(not(release))]
const RUST_LOG_VALUE: &str = "debug";

#[cfg(release)]
const RUST_LOG_VALUE: &str = "error";

#[derive(Deserialize)]
pub struct Config {
    #[cfg(feature = "tcp_edge")]
    pub tcp_edge: TcpEdge,
}

#[derive(Deserialize)]
pub struct TcpEdge {
    #[cfg(feature = "tcp_edge")]
    pub bms: Bms,

    #[cfg(feature = "tcp_edge")]
    pub emg: Emg,
}

#[derive(Deserialize)]
pub struct Bms {
    #[cfg(feature = "tcp_edge")]
    pub host: String,

    #[cfg(feature = "tcp_edge")]
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Emg {
    #[cfg(feature = "tcp_edge")]
    pub host: String,

    #[cfg(feature = "tcp_edge")]
    pub port: u16,
}

pub fn init() -> Result<Config> {
    let config_path = var(CONFIG_PATH_KEY)?;
    let config_path = format!("config/{}.yaml", config_path);
    set_var(RUST_LOG_KEY, RUST_LOG_VALUE);
    pretty_env_logger::try_init()?;
    let file = File::with_name(&config_path);
    let config = config::Config::builder()
        .add_source(file)
        .build()?
        .try_deserialize::<Config>()?;
    Ok(config)
}
