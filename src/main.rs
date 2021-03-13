/* external crates */

/* external uses */
// use pyo3::{
//     prelude::*,
//     types::{
//         IntoPyDict,
//         PyModule
//     },
// };
// use actix::prelude::*;
// use actor::{
//     status_actor::CriticalActor,
//     user_actor::NonCriticalActor,
//     // ping::Ping,
// };
use std::fs::File;
use actix::prelude::*;
use crate::messages::{
    actuator::{
        contract,
        send_home,
        stop
    },
    diagnostics::check:: {
        CheckResponse,
        Check

    }
};
use crate::messages::response::{
    Response:: {
        Accepted,
        Rejected
    },
    Rejected:: {
        EventLoopTooFull,
        InvalidState,
        Other
    }
};

/* internal crates */
mod actor;
mod json_io;
mod messages;

/* internal uses */

fn create_file(file_name: &str) -> () {
    match File::create(format!("./py_io/{}", file_name)) {
        Ok(_) => (),
        Err(_) => panic!(),
    };
}

#[actix_rt::main] 
async fn main() -> () {
    create_file("input.json");
    create_file("output.json");

    let system = System::new("Test");

    let arbiter_1 = Arbiter::new();
    let arbiter_2 = Arbiter::new();
    let status_actor = actor::status_actor::StatusActor::start_in_arbiter(&arbiter_1, move |_ctx: &mut Context<actor::status_actor::StatusActor>| actor::status_actor::StatusActor::new());
    let userActor = actor::user_actor::UserActor::start_in_arbiter(&arbiter_2, move |_ctx: &mut Context<actor::user_actor::UserActor>| actor::user_actor::UserActor::new());


    let diagnostics_send = status_actor.send(Check).await;

    match diagnostics_send {
        Ok(res) => {
            match res.unwrap() {
                Accepted(result) => {
                    println!("{}", result.battery_percentage);
                }
                Rejected(rejected) => {
                    match rejected {
                        EventLoopTooFull => {
                            println!("Event loop too full");
                        }
                        InvalidState => {
                            println!("Arm's current state does not support request");
                        }
                        Other => {
                            println!("bla");
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Event loop is too full");
        }
    }

    // let critical_actor = CriticalActor.start();
    // let non_critical_actor = NonCriticalActor.start();

    // let result_non_crit = non_critical_actor.send(Ping::A).await;
    // let result = critical_actor.send(Ping::B).await;

    // deserial_json();
    //read_from_file("input.json");

    //instance();
    //write_json("input.json");
    //instance();
    //read_from_file("input.json");

    // instance();
    // write_json("input.json");
    // instance();

    // let result = monitor_actor.send(Read).await;

    // assert!(result.unwrap() == true);
    // println!("{}", result.unwrap());

}

#[cfg(test)]
mod main_test {
    use super::*;

    #[test]
    fn init() -> () {
        create_file("input.json");
        create_file("output.json");
    }
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

//         test.call0("my_function")?;
//         let test2: f64 = test.call0("my_function2")?.extract()?;
//         let test3: f64 = test.call1("my_function3", (3,))?.extract()?;

//         // println!("{}, {}", test2, test3);

//         let relu_result: f64 = activators.call1("relu", (-1.0,))?.extract()?;
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
