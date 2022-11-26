use anyhow::Result;

#[derive(Default)]
pub(super) struct State;

#[cfg(feature = "pseudo_analytics")]
pub(super) fn parser(_: &mut State, _: f64) -> Result<()> {
    Ok(())
}

#[cfg(not(feature = "pseudo_analytics"))]
pub(super) fn parser(_: &mut State, _: f64) -> Result<()> {
    todo!()
}
