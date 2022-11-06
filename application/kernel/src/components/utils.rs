#[cfg(feature = "simulation")]
use std::ops::Range;

use anyhow::Result;
#[cfg(feature = "simulation")]
use crossbeam::channel::Sender;
#[cfg(feature = "simulation")]
use log::error;
#[cfg(feature = "simulation")]
use log::info;
#[cfg(feature = "simulation")]
use log::warn;

pub fn parse_float(buffer: &[u8]) -> Result<f64> {
    let float = String::from_utf8_lossy(buffer).parse::<f64>()?;
    Ok(float)
}

#[cfg(feature = "simulation")]
pub fn buffer_check<T>(
    tx: &Sender<T>,
    name: &'static str,
    response_capacity: usize,
    warning_interval: Range<usize>,
) {
    let len = tx.len();
    match len {
        _ if len == response_capacity => error!("{} length: {}", name, len),
        _ if warning_interval.contains(&len) => warn!("{} length: {}", name, len),
        _ => info!("{} length: {}", name, len),
    }
}
