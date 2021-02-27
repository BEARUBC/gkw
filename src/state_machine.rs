//use crate::Interface;

type DriverIDType = u8;
const DEFAULT_DRIVER_ID: DriverIDType = 0u8;

trait Interface {
    fn a(self: &mut Self) -> ();
    fn b(self: &mut Self) -> ();
}

#[derive(Debug)]
pub enum State {
    Safety,
    Active,
    Failure,
}

#[derive(Debug)]
pub struct Machine {
    pub(crate) current_state: State,
    driver_id: DriverIDType,
}

impl Machine {
    pub(crate) fn new() -> Machine {
        return Machine {
            current_state: State::Active,
            driver_id: DEFAULT_DRIVER_ID,
        };
    }

    pub(crate) fn transition(self: &mut Self, driver_id: DriverIDType) {
        match &(*self).current_state {
            State::Safety => {
                /*State::Safety => todo!(),
                State::Active => todo!(),
                State::Failure => todo!(),*/
                todo!()
            },
            State::Active => {
                /*State::Safety => todo!(),
                State::Active => true,
                State::Failure => todo!(),*/
                self.current_state = State::Safety;
            },
            State::Failure => {
                /*State::Safety => todo!(),
                State::Active => todo!(),
                State::Failure => panic!(),*/
                todo!()
            },
        }

    }
}
impl Interface for Machine {
    fn a(self: &mut Self) -> () {
        match &(*self).current_state {
            State::Safety => {},
            State::Active => {},
            State::Failure => {},
        };
    }
    fn b(self: &mut Self) -> () {
        match &(*self).current_state {
            State::Safety => {},
            State::Active => {},
            State::Failure => {},
        };
    }
}
