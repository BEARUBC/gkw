use std::{time, thread, env};
use std::process::{Command, Stdio, Child};
mod lib

fn main() {

    let addresses = [0x80,0x81,0x82,0x83];
    let location = 6; 

    let mut python_process = Analytics::new("application\python_integration\python\wrapper.py").expect("Failed to start wrapper");
        let res = python_process
            .make_request("movement".to_string(), Value::String("hello".to_string()), location).expect("Failed to make request");
        assert_eq!(json!("HELLO"), res);
}
