/* external crates */

/* external uses */
use std::io::Error;

/* internal mods */

/* internal uses */
use super::{
    interface::Interface,
    states::States,
};

pub(in super::super) struct Machine {
    current_state: States,
}

impl Machine {
    pub(in super::super) fn new() -> Self {
        todo!();
    }

    #[allow(unused)]
    pub(in super::super) fn transition(self: &mut Self, next: States) -> Result<(), Error> {
        todo!();
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
