/* external crates */

/* external uses */
// use pyo3::{
//     prelude::*,
//     types::{
//         IntoPyDict,
//         PyModule
//     },
// };
use std::fs::File;
use actix::prelude::*;

/* internal crates */
mod actor;
mod json_io;
mod messages;

/* internal uses */
#[allow(unused_imports)]
use crate::actor::{
    status_actor::StatusActor,
    user_actor::UserActor,
};

#[allow(unused)]
fn create_file(file_name: &str) -> () {
    match File::create(format!("./py_io/{}", file_name)) {
        Ok(_) => (),
        Err(_) => panic!(),
    };
}

fn main() -> () {
    let system = System::new("Grasp -- main_binary: v0.0.1");
    Arbiter::spawn(async {
    });
    System::current().stop();
    
    match system.run() {
        Ok(()) => (),
        Err(_) => (),
    }

    // let _ = System::run(|| {

    // });

    // create_file("input.json");
    // create_file("output.json");

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
