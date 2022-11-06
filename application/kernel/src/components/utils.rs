#[cfg(feature = "simulation")]
use anyhow::Result;

#[cfg(feature = "simulation")]
pub(super) fn parse_float(buffer: &[u8]) -> Result<f64> {
    let float = String::from_utf8_lossy(buffer).parse::<f64>()?;
    Ok(float)
}
