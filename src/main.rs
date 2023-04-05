use std::env;

use emg_integration;
use messages;
use python_integration::*;
use serde_json::json;
use serde_json::value::Value;

const EMG_DATA_LEN: u32 = 10;

fn main() {
    let mut python_process = Analytics::new("python/wrapper.py", "").expect("Failed to start wrapper");
    let res = python_process
        .make_request("m_emg".to_string(), Value::String("{\"emg_buffer\": [[[0]]]}".to_string())).expect("Failed to make request");
    let data = r#"
    {
        "contractions": [0.0]
    }"#;
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data).unwrap();
    println!("{}", res);
    assert_eq!(v, res);

    // let mut emg = emg_integration::Emg::new()
    // let mut emg = python_integration::Analytics::new("application/emg_collection.py").expect("Failed to start EMG");
   
   
    // let mut analytics_emg = Analytics::new("./python/wrapper.py").expect("Failed to start wrapper");
    // let mut analytics_camera =
    //     Analytics::new("./python/wrapper.py").expect("Failed to start wrapper");

    // loop {
    //     let emg_data = emg_integration.get_data_queue(EMG_DATA_LEN);

    //     let emg_req_json = json!({ "emg_buffer": format!("{}", emg_data) });

    //     let emg_res = analytics_emg
    //         .make_request("m_emg".to_string(), emg_req_json.to_string())
    //         .expect("Failed EMG Request");

    //     let camera_res = analytics_camera.make_request("camera".to_string());
    // }
}
