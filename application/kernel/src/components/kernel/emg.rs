use anyhow::Result;
use raestro::maestro::constants::Channels;
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
            let channels = [
                Channels::Channel0,
                Channels::Channel1,
                Channels::Channel2,
                Channels::Channel3,
                Channels::Channel4,
                Channels::Channel5,
            ];
            let positions: [u16; 3usize] = grip.into();
            for channel in 0..3 {
                maestro.set_target(channels[channel], positions[channel])?;
            }
            Ok(())
        },
    }
}

#[cfg(not(feature = "pseudo_analytics"))]
pub(super) fn parser(_: &mut State, _: f64) -> Result<()> {
    todo!()
}
