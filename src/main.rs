#![allow(unused)]
#![allow(dead_code)]

// use pyo3::{
//     prelude::*,
//     types::{
//         IntoPyDict,
//         PyModule
//     },
// };
use std::fs;
use actix::prelude::*;
use actor::{
    critical_actor::CriticalActor,
    non_critical_actor::NonCriticalActor,
    ping::Ping,
};
use rppal::i2c::I2c;

mod state_machine;
mod actor;
mod stepper;
/*use stepper::{
    i2c_interface::I2C,
};*/
use stepper::i2c_interface::I2C;

//use crate::actor::critical_actor::CriticalActor;

#[actix_rt::main] 
async fn main() {

    let critical_actor = CriticalActor.start();
    let non_critical_actor = NonCriticalActor.start();

    let result_non_crit = non_critical_actor.send(Ping::A).await;
    let result = critical_actor.send(Ping::B).await;

    // this block of code will likely give errors. Need to test by building first.
    // let mut i2c: I2c = create_i2c().i2c;
    let mut i2c: I2c = I2C::new().i2c;
    // let mut read_buffer: Vec<u8> = vec![];
    let mut buffer: [u8; 1024] = [0x0u8; 1024];

    let read_bits = i2c.read(&mut buffer);

    // println!(read_buffer);
    // let mut write_buffer:Vec<u8> = vec![];
    //i2c::write(&mut write_buffer);


    // let result = monitor_actor.send(Read).await;

    // assert!(result.unwrap() == true);
    // println!("{}", result.unwrap());

}

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

// fn main() -> PyResult<()> {
//     // let mut machine_test = state_machine::Machine::new();
//     // machine_test.transition(0u8, state_machine::State::Safety);
//     // println!("{:?}", machine_test);

//     return Python::with_gil(|py| {
//         let filename: &str = "py/test.py";
//         let contents: String = fs::read_to_string(filename).expect("error reading the python file");

//         let test = PyModule::from_code(py, &contents, "test.py", "test")?;

//         // test.call0("my_function")?;
//         // let test2: f64 = test.call0("my_function2")?.extract()?;
//         // let test3: f64 = test.call1("my_function3", (3,))?.extract()?;

//         // println!("{}, {}", test2, test3);

//         // let relu_result: f64 = activators.call1("relu", (-1.0,))?.extract()?;
//         // assert_eq!(relu_result, 0.0);

//         // let kwargs = [("slope", 0.2)].into_py_dict(py);
//         // let lrelu_result: f64 = activators
//         //     .call("leaky_relu", (-1.0,), Some(kwargs))?
//         //     .extract()?;
//         // assert_eq!(lrelu_result, -0.2);
//         println!("asdf");
//         return Ok(());
//     });
// }
