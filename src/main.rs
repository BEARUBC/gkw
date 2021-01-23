#![allow(unused)]
#![allow(dead_code)]

mod state_machine;
use pyo3::{
    prelude::*,
    types::{
        IntoPyDict,
        PyModule
    },
};
use std::fs;

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


fn main() -> PyResult<()> {
    // let mut machine_test = state_machine::Machine::new();
    // machine_test.transition(0u8, state_machine::State::Safety);
    // println!("{:?}", machine_test);

    return Python::with_gil(|py| {
        let filename: &str = "py/test.py";
        let contents: String = fs::read_to_string(filename).expect("error reading the python file");

        let test = PyModule::from_code(py, &contents, "test.py", "test")?;

        // test.call0("my_function")?;
        // let test2: f64 = test.call0("my_function2")?.extract()?;
        // let test3: f64 = test.call1("my_function3", (3,))?.extract()?;

        // println!("{}, {}", test2, test3);

        // let relu_result: f64 = activators.call1("relu", (-1.0,))?.extract()?;
        // assert_eq!(relu_result, 0.0);

        // let kwargs = [("slope", 0.2)].into_py_dict(py);
        // let lrelu_result: f64 = activators
        //     .call("leaky_relu", (-1.0,), Some(kwargs))?
        //     .extract()?;
        // assert_eq!(lrelu_result, -0.2);
        println!("asdf");
        return Ok(());
    });
}
