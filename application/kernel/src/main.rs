use std::process::exit;

use log::error;
use log::info;

use serde_json::value::Value;

use python_integration::Analytics;

mod init;
mod kernel;

/// Main entrypoint into the GKW software.
///
/// The default number of threads on our Raspberry Pi is 4.
/// Therefore, without hyper-threading the CPU, the number of native-threads that should be
/// instantiated is 4.
// #[allow(unreachable_code)]
// #[tokio::main(flavor = "multi_thread", worker_threads = 4)]
// async fn main() -> ! {
//     init::init_logging();

//     match init::init().await {
//         Ok(()) => info!("GKW initialization has succeeded."),
//         Err(err) => {
//             error!("GKW initialization has failed with error: {}.", err);
//             exit(1);
//         },
//     };

//     #[allow(clippy::empty_loop)]
//     loop {
        
//     }

//     error!("The GKW main-loop has crashed unexpectedly. Exiting...");
//     unreachable!()
// }

//Shuyao version

fn main() {
    // let python_process_res = Analytics::new("./application/python_integration/python/wrapper.py");
    // let mut python_process = if let Ok(python_process) = python_process_res{
    //     python_process
    // } else {
    //     panic!("Failed to start wrapper");
    // };

    // let result = python_process.make_request("add_ten".to_string(), Value::String(10.to_string()));
    // let res = if let Ok(res) = result {
    //     res
    // } else {
    //     println!("Failed, error: \"{:?}\"", result);
    //     "failed".to_string()
    // };

    // println!("{}", res);
}

fn main2() {
// setup the instance of the two structs, emg and our struct
// get data from emg
// call analytics function
// pull raestro in
}