/* external crates */

/* external uses */

/* internal mods */

/* internal uses */
use crate::state_machine::{
    interface::Interface,
    states::States,
};

type DriverIDType = u8;
#[allow(unused)]
const DEFAULT_DRIVER_ID: DriverIDType = 0u8;

#[allow(dead_code)]
pub struct Machine {
    current_state: States,
    driver_id: DriverIDType,
}

impl Machine {
    #[allow(unused)]
    pub(crate) fn new() -> Machine {
        return Machine {
            current_state: States::Active,
            driver_id: DEFAULT_DRIVER_ID,
        };
    }

    #[allow(unused)]
    pub(crate) fn transition(self: &mut Self, driver_id: DriverIDType, next_state: States) -> bool {
        return match &(*self).current_state {
            States::Safety => match next_state {
                States::Safety => todo!(),
                States::Active => todo!(),
                States::Failure => todo!(),
            },
            States::Active => match next_state {
                States::Safety => todo!(),
                States::Active => true,
                States::Failure => todo!(),
            },
            States::Failure => match next_state {
                States::Safety => todo!(),
                States::Active => todo!(),
                States::Failure => panic!(),
            },
        };
    }
}

impl Interface for Machine {
    fn a(self: &mut Self) -> () {
        match &(*self).current_state {
            States::Safety => {},
            States::Active => {},
            States::Failure => {},
        };
    }
    fn b(self: &mut Self) -> () {
        match &(*self).current_state {
            States::Safety => {},
            States::Active => {},
            States::Failure => {},
        };
    }
}
