use anyhow::Result;
use raestro::Maestro;

use crate::components::kernel::grip::Grip;
pub struct State {
    current_grip: Grip,
}

impl State {
    pub fn new() -> Self {
        Self {
            current_grip: Grip::default(),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}


#[cfg(feature = "pseudo_analytics")]
pub(super) fn parser(m: &mut Maestro, state: &mut State, data: f64) -> Result<()> {
    use raestro::prelude::Channels;

    let next_grip = Grip::from(data);

    if state.current_grip == next_grip {
        return Ok(());
    }

    let channels = [
        Channels::C_0,
        Channels::C_1,
        Channels::C_2,
        Channels::C_3,
        Channels::C_4,
        Channels::C_5,
    ];

    let positions: [u16; 3usize] = next_grip.into();

    for i in 0..3 {
        m.set_target(channels[i], positions[i]).unwrap();
    }

    Ok(())
}

#[cfg(not(feature = "pseudo_analytics"))]
pub(super) fn parser(_: &mut State, _: f64) -> Result<()> {
    todo!()
}
