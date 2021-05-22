/* external crates */
use tokio::runtime::Builder;
use std::{
    thread,
    time::Duration,
};

/* external uses */
// use pyo3::{
//     prelude::*,
//     types::{
//         IntoPyDict,
//         PyModule
//     },
// };

/* internal crates */
mod json_io;
mod messages;
use async_framework::prelude::*;

/* internal uses */
#[allow(unused_imports)]
use crate::{
    messages::{
        actuator::{
            contract,
            send_home,
            stop,
        },
        diagnostics::{
            check::{
                CheckResponse,
                Check,
            },
            ping::Ping,
        },
        response::{
            Response:: {
                Accepted,
                Rejected,
            },
            Rejected:: {
                EventLoopTooFull,
                InvalidState,
                Other,
            },
        }
    },
};

/* internal mods */
fn main() -> () {

    /*let custom = Component::<Asynchronous>::new(String::from("custom"), handler);

    Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            loop {
                thread::sleep(Duration::from_secs(3u64));

                custom
                    .send(Asynchronous::Variant1)
                    .unwrap();
                custom
                    .send(Asynchronous::Variant2(EmbeddedData))
                    .unwrap();
            };
        });
    */

    // create_file("input.json");
    // create_file("output.json");

    // let system = System::new("Test");

    // let arbiter_1 = Arbiter::new();
    // let arbiter_2 = Arbiter::new();
    // let status_actor = actor::status_actor::StatusActor::start_in_arbiter(&arbiter_1, move |_ctx: &mut Context<actor::status_actor::StatusActor>| StatusActor::new());
    // let user_actor = actor::user_actor::UserActor::start_in_arbiter(&arbiter_2, move |_ctx: &mut Context<actor::user_actor::UserActor>| UserActor::start());

    // let diagnostics_send = status_actor.send(Check).await;

    // match diagnostics_send {
    //     Ok(res) => {
    //         match res.unwrap() {
    //             Accepted(result) => {
    //                 println!("{}", result.battery_percentage);
    //             }
    //             Rejected(rejected) => {
    //                 match rejected {
    //                     EventLoopTooFull => {
    //                         println!("Event loop too full");
    //                     }
    //                     InvalidState => {
    //                         println!("Arm's current state does not support request");
    //                     }
    //                     Other => {
    //                         println!("bla");
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         println!("Event loop is too full");
    //     }
    // }
}

#[cfg(test)]
mod main_test {
    use std::fs::File;
    #[allow(unused)]
    fn create_file(file_name: &str) -> () {
        match File::create(format!("./io/{}", file_name)) {
            Ok(_) => (),
            Err(_) => panic!(),
        };
    }

    #[test]
    fn init() -> () {
        create_file("input.json");
        create_file("output.json");
    }
}

// mod python;

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
