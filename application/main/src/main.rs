#[path = "../../emg_integration/src/lib.rs"] mod EMG_INTEGRATION;
#[path = "../../python_integration/src/lib.rs"] mod Analytics;
#[path = "../../motors/src/lib.rs"] mod Motor;
use serde_json::json;

const EMG_DATA_LEN: u32 = 10;

fn main() {
    let mut emg_integration: EMG_INTEGRATION = EMG_INTEGRATION::new("../scripts/emg_collection.py", 10).expect("Failed to start EMG");
    let mut analytics_emg = Analytics::new("./python/wrapper.py").expect("Failed to start wrapper");
    let mut analytics_camera = Analytics::new("./python/wrapper.py").expect("Failed to start wrapper");

    loop {
        let emg_data = emg_integration.get_data_queue(EMG_DATA_LEN);

        let emg_req_json = json!({
            "emg_buffer": format!("{}", emg_data);
        });

        let emg_res = analytics_emg.make_request(
            "m_emg".to_string(),
            emg_req_json.to_string()
        ).expect("Failed EMG Request");

        let camera_res = analytics_camera.make_request(
            "camera".to_string(),
            
        )


        
        

    }


}
