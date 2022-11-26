use anyhow::Result;

use crate::components::kernel::State;

#[cfg(feature = "pseudo_analytics")]
fn parser(_: &mut State, _: f64) -> Result<()> {
    Ok(())
}

#[cfg(not(feature = "pseudo_analytics"))]
fn parser(state: &mut State, _: f64) -> Result<()> {
    todo!()
}
