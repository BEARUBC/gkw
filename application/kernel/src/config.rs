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
    pub components: Components,
}

#[derive(Deserialize)]
pub struct Components {
    #[cfg(feature = "tcp_edge")]
    pub bms: TcpComponent,

    #[cfg(feature = "tcp_edge")]
    pub emg: TcpComponent,

    #[cfg(feature = "tcp_edge")]
    pub fsr: TcpComponent,
}

#[cfg(feature = "tcp_edge")]
#[derive(Deserialize)]
pub struct TcpComponent {
    pub host: String,
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
