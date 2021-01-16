#![allow(unused)]
#![allow(dead_code)]

type DriverIDType = u8;
const DEFAULT_DRIVER_ID: DriverIDType = 0u8;

trait Interface {
    fn a(self: &mut Self) -> ();
    fn b(self: &mut Self) -> ();
}

// struct Safety;
// struct Active;
// struct Failure;

// impl Interface for Safety {
//     fn a() -> () {}
//     fn b() -> () {}
// }
// impl Interface for Active {
//     fn a() -> () {}
//     fn b() -> () {}
// }
// impl Interface for Failure {
//     fn a() -> () {}
//     fn b() -> () {}
// }

pub enum State {
    Safety,
    Active,
    Failure,
}

pub struct Machine {
    current_state: State,
    driver_id: DriverIDType,
}
impl Machine {
    fn new() -> Machine {
        return Machine {
            current_state: State::Active,
            driver_id: DEFAULT_DRIVER_ID,
        };
    }

    fn transition(self: &mut Self, driver_id: DriverIDType, next_state: State) -> bool {
        return match &(*self).current_state {
            State::Safety => match next_state {
                State::Safety => todo!(),
                State::Active => todo!(),
                State::Failure => todo!(),
            },
            State::Active => match next_state {
                State::Safety => todo!(),
                State::Active => true,
                State::Failure => todo!(),
            },
            State::Failure => match next_state {
                State::Safety => todo!(),
                State::Active => todo!(),
                State::Failure => panic!(),
            },
        };
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

fn main() -> () {}
