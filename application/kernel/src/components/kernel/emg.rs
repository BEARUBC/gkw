use anyhow::Result;
use raestro::maestro::constants::Channel;
use raestro::maestro::Maestro;

use crate::components::kernel::grip::Grip;

#[derive(Default)]
pub struct State {
    grip: Grip,
}

#[cfg(feature = "pseudo_analytics")]
pub(super) fn parser(maestro: &mut Maestro, state: &mut State, data: f64) -> Result<()> {
    let grip = data.into();
    match state.grip == grip {
        true => Ok(()),
        false => {
            let channels = [Channel::Channel0, Channel::Channel1, Channel::Channel2];
            let targets: [u16; 3usize] = grip.into();
            channels
                .into_iter()
                .zip(targets)
                .try_for_each(|(channel, target)| maestro.set_target(channel, target))?;
            Ok(())
        },
    }
}

#[cfg(not(feature = "pseudo_analytics"))]
pub(super) fn parser(_: &mut State, _: f64) -> Result<()> {
    todo!()
}
