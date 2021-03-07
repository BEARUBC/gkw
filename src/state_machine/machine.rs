/* external crates */

/* external uses */

/* internal mods */

/* internal uses */
use super::{
    interface::Interface,
    states::States,
};

pub(in super) struct Machine {
    pub(in super) current_state: States,
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
