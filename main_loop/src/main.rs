use python_integration::{*};
use std::env;

fn main() {
    println!("Hello, world!");

    let mut python_process = Analytics::new("../../../grasp-py/src/grasp_analytics/modules/emg/emg.py").expect("Failed to start wrapper");

}

