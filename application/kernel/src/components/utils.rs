use std::ops::Range;

use anyhow::Result;
use crossbeam::channel::Sender;
use log::error;
use log::info;
use log::warn;

pub fn parse_float(buffer: &[u8]) -> Result<f64> {
    let float = String::from_utf8_lossy(buffer).parse::<f64>()?;
    Ok(float)
}

pub fn buffer_check<T>(tx: &Sender<T>, response_capacity: usize, warning_interval: Range<usize>) {
    let len = tx.len();
    match len {
        _ if len == response_capacity => error!("length: {}", len),
        _ if warning_interval.contains(&len) => warn!("length: {}", len),
        _ => info!("length: {}", len),
    }
}
