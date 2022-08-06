use std::process::exit;

use log::error;
use log::info;

use serde_json::value::Value;

use python_integration::json;
use python_integration::Analytics;
use emg_integration::EMG_INTEGRATION;
//Note: originally wouldn't work cause it was named library.rs not lib.rs

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
    //Setup Analytics struct
    let python_process_res = Analytics::new("./application/python_integration/python/wrapper.py");
    let mut python_process = if let Ok(python_process) = python_process_res{
        python_process
    } else {
        panic!("Failed to start wrapper");
    };

    //Setup emg struct
    let emg_res = EMG_INTEGRATION::new(".../emg_integration/python/test.py", 10);
    let mut emg = if let Ok(emg) = emg_res{
        emg
    } else {
        panic!("Failed to start emg");
    };

    //Call Analytics function
    let result = python_process.make_request("capitalize".to_string(), Value::String("hello".to_string()));
    let res = if let Ok(res) = result {
        res
    } else {
        println!("Failed, error: \"{:?}\"", result);
        json!("failed".to_string())
    };

    //Get data from emg
    let emg_data_res = emg.get_data_queue(9);
    let emg_data = if let Ok(emg_data) = emg_data_res {
        emg_data
    } else {
        println!("Failed, error: \"{:?}\"", emg_data_res);
        json!("failed".to_string())
    };
            
}

// setup the instance of the two structs, emg and our struct
// get data from emg
// call analytics function
// pull raestro in