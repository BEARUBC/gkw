use anyhow::Result;
use raestro::maestro::Maestro;
use raestro::maestro::constants::Channels;

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
            match grip {
                Grip::Cup => {
                    maestro.set_target(channels[0], 300u16).unwrap();
                },
                Grip::Hammer => {
                    maestro.set_target(channels[0], 150u16).unwrap();
                },
                Grip::Flat => {
                    maestro.set_target(channels[0], 0u16).unwrap();
                },
            }
            todo!()
        },
    }
    // if state.grip == grip {
    //     return Ok(());
    // }
    // Ok(())
}

#[cfg(not(feature = "pseudo_analytics"))]
pub(super) fn parser(_: &mut State, _: f64) -> Result<()> {
    todo!()
}
