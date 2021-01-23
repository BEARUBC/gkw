#![allow(unused)]
#![allow(dead_code)]

mod state_machine;
// mod python;


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


fn main() -> () {
    let mut machine_test = state_machine::Machine::new();
    machine_test.transition(0u8, state_machine::State::Safety);
    println!("{:?}", machine_test);
}
