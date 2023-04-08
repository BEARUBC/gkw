use emg_integration;
use messages;
use motors::{self, Grip};
use python_integration::*;
use serde_json::json;
use serde_json::value::Value;
use std::env;
use std::error::Error;
use std::{thread, time};

const EMG_DATA_LEN: u32 = 10;
const EMG_ACTIVATION_THRESHOLD: f64 = 0.33;

fn read_json(raw_json: &str) -> Value {
    let parsed: Value = serde_json::from_str(raw_json).unwrap();
    return parsed;
}

fn motor_safety(from: i64) {
    println!("Setting motor to safety state {}", from);
    // if motors fail
    // Panic!;
}

fn set_grip_type(gripType: Grip) {
    println!("set_grip_type {:?}", gripType)
}

fn main() {
    // let emg_integration = emg_integration::Emg::new("python/data_to_stdout_txt.py", 10)
    //     .expect("Failed to start emg reader script");
     let mut roboclaw_test =
            Analytics::new("python/wrapper.py", ".\python").expect("Failed to start wrapper");
        let rest = roboclaw_test
            .make_request(
                "motor_test".to_string(),
                Value::String("{\"yes\": [[[0]]]}".to_string()),
            )
            .expect("Failed to make request");

   
//    let mut python_process =
//             Analytics::new("python/wrapper.py", "../../").expect("Failed to start wrapper");
//         let res = python_process
//             .make_request(
//                 "m_emg".to_string(),
//                 Value::String("{\"emg_buffer\": [[[0]]]}".to_string()),
//             )
//             .expect("Failed to make request");
//         let data = r#"
//         {
//             "contractions": [0.0]
//         }"#;  let mut python_process =
//         Analytics::new("python/wrapper.py", "").expect("Failed to start wrapper");

//     let mut motor_controller = motors::Motor::new(1).expect("Failed to create motor");

//     for _n in 1..=4 {
//         // let emg_data = emg_integration.get_data_queue(EMG_DATA_LEN).unwrap();
//         // let emg_req_json = json!({ "emg_buffer": format!("{:?}", emg_data) });

//         let emg_json = python_process
//             .make_request(
//                 "m_emg".to_string(),
//                 Value::String("{\"emg_buffer\": [[[0,1]]]}".to_string()),
//             )
//             .expect("Failed to make request");
//         let emg_opt = &emg_json.get("contractions").unwrap()[0].as_f64();
//         let mut emg_val = 0.0;
//         match emg_opt {
//             Some(emg_opt) => emg_val = *emg_opt,
//             None => motor_safety(0),
//         }

//         println!("{}\n", emg_val);
//         if emg_val < EMG_ACTIVATION_THRESHOLD {
//             println!("CONTINUING");
//             continue;
//         }

//         let cv_json = python_process
//             .make_request("add_ten".to_string(), Value::String("-9".to_string()))
//             .expect("Failed to make request");
//         // let cv_opt = &cv_json.get("grip_type").unwrap().as_str();
//         let cv_opt = &cv_json.as_f64();

//         let mut cv_val = Grip::default();
//         match cv_opt {
//             Some(cv_opt) => cv_val = motors::Grip::from(*cv_opt),

//             // match cv_opt {
//             //     cv_val = motors::Grip::from(cv_opt);
//             //     // &"0" => cv_val = GripTypes::zero,
//             //     // &"1" => cv_val = GripTypes::one,
//             //     // &"2" => cv_val = GripTypes::two,
//             //     // &"3" => cv_val = GripTypes::three,
//             //     // &"4" => cv_val = GripTypes::four,
//             //     0 => cv_val = GripTypes::zero,
//             //     1 => cv_val = GripTypes::one,
//             //     2 => cv_val = GripTypes::two,
//             //     3 => cv_val = GripTypes::three,
//             //     4 => cv_val = GripTypes::four,
//             //     _ => motor_safety(1),
//             // },
//             None => motor_safety(2),
//         }
//         motor_controller.changeGrip(1.1);

//         println!("DONE")
    }

    let ten_millis = time::Duration::from_millis(10);

    thread::sleep(ten_millis);

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
