use serde::{Deserialize, Serialize};
pub use serde_json::json;
use serde_json::value::Value;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{format, thread, time};
use uuid::Uuid;
// use messages::Custom_log;
use anyhow::anyhow;
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    request_id: Uuid,
    request_type: String,
    params: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    request_id: Uuid,
    valid_bit: i8,
    data: Value,
}

pub struct Analytics {
    child_process: Child,
    stdin: ChildStdin,
    read_thread: JoinHandle<()>,
    response_holder: Arc<Mutex<HashMap<Uuid, Option<Response>>>>,
}

impl Analytics {
    //Description: Begins the process of getting a response from the Python code. Starts the child
    //process then loops until it gets a response. Once it recieves a response it puts it in storage
    //Parameters: child_process - name of the child process
    //Returns: Analytics struct

    pub fn new(python_process: &str, path_to_analytics: &str) -> Result<Analytics> {
        // let python_log_res = Custom_log::new();
        // let mut python_log = if let Ok(python_log) = python_log_res{
        //     python_log
        // } else {
        //     panic!("Failed to start log");
        // };
        // python_log.write_log(b"starting log");
        // start child process

        let path_to_script = format!("{}{}", path_to_analytics, python_process);
        let mut child_process = Command::new("python")
            .args([path_to_script])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        //get stdin and stdout and verify it works
        let mut stdin = child_process
            .stdin
            .take()
            .ok_or_else(|| anyhow!("Failed to get stdin"))?;
        let stdout = child_process
            .stdout
            .take()
            .ok_or_else(|| anyhow!("Failed to get stdout"))?;

        // let mut owned_string: String = "{\"path_to_analytics\": ".to_owned();
        // let another_owned_string: String = path_to_analytics.to_owned();
        // owned_string.push_str(&another_owned_string);
        let prelude = "{\"path_to_analytics\": \"";
        let epilogue = "\"}\n";
        let json_string = format!("{}{}{}", prelude, path_to_analytics, epilogue);
        stdin.write_all(json_string.as_bytes())?;

        //create a map and a map clone for the data
        let storage = Arc::new(Mutex::new(HashMap::new()));
        let storage_clone = Arc::clone(&storage);

        //Begin the read thread
        let read_thread = thread::spawn(move || {
            let mut f = BufReader::new(stdout);
            loop {
                let mut resp_string = String::new();

                //read from the stdout using Bufreader and verify it works
                let check_read = f.read_line(&mut resp_string);
                if let Err(_e) = check_read {
                    //continue if it fails, to avoid an infinite loop it will eventually timeout
                    continue;
                }

                //check if there was a response or if there was nothing
                if resp_string != "" {
                    //turn the string from Python into a Response struct and verify it works
                    let response_packet: Response = match serde_json::from_str(&resp_string) {
                        Ok(response_packet) => response_packet,
                        Err(_e) => {
                            //continue if it fails
                            continue;
                        },
                    };

                    //get access to the map and verify it works
                    let mut locked_map_clone = match storage_clone.lock() {
                        Ok(locked_map_clone) => locked_map_clone,
                        Err(_e) => {
                            //continue if it fails
                            continue;
                        },
                    };

                    //insert the response into the map
                    locked_map_clone
                        .insert(response_packet.request_id, Option::Some(response_packet));
                }
            }
        });

        // python_log.write_log(b"Generated Analytics Struct");
        return Ok(Analytics {
            child_process,
            stdin,
            read_thread,
            response_holder: storage,
        });
    }

    //Description: Takes a function request and parameters and sends instructions to Analytics code that
    //fulfills the request. After sending the request it will loop until it finds the reponse in storage
    //then returns the response
    //Parameters: func - string that specifies the function
    //            parameters - map that holds each parameter name and value
    //Returns: String version of the response
    pub fn make_request(&mut self, func: String, parameters: Value) -> Result<Value> {
        // make the request object
        let my_uuid = Uuid::new_v4();
        let request_packet = Request {
            request_id: my_uuid,
            request_type: func,
            params: parameters,
        };

        let json_string = serde_json::to_string(&request_packet)? + "\n";

        // make space in map
        {
            let mut locked_map = self
                .response_holder
                .lock()
                .map_err(|e| anyhow!("Unable to make space in map. Error: {}", e))?;
            locked_map.insert(my_uuid, None);
        }

        println!("Sending {}", json_string);
        // send string over
        let result = self.stdin.write_all(json_string.as_bytes())?;
        println!("Sent Data");

        // wait for child process to finish processing
        loop {
            let locked_map = self
                .response_holder
                .lock()
                .map_err(|e| anyhow!("Cannot lock map. Error: {}", e))?;

            let val = locked_map
                .get(&my_uuid)
                .ok_or_else(|| anyhow!("Failed to get value in map"))?;
            if let Some(res) = val {
                return Ok(res.data.clone());
            }
            let ten_millis = time::Duration::from_millis(10);
            thread::sleep(ten_millis);
        }
    }
    // TODO:
    // - When checking response eventually timeout
}

// Note: to run all tests in the python_integration package, run:
// cargo test -p python_integration
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emg() {
        let mut python_process =
            Analytics::new("python/wrapper.py", "../../").expect("Failed to start wrapper");
        let res = python_process
            .make_request(
                "m_emg".to_string(),
                Value::String("{\"emg_buffer\": [[[0]]]}".to_string()),
            )
            .expect("Failed to make request");
        let data = r#"
        {
            "contractions": [0.0]
        }"#;
        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(data).unwrap();
        assert_eq!(v, res);
    }

    #[test]
    fn test_capitalize() {
        let mut python_process =
            Analytics::new("python/wrapper.py", "../../").expect("Failed to start wrapper");
        let res = python_process
            .make_request("capitalize".to_string(), Value::String("hello".to_string()))
            .expect("Failed to make request");
        assert_eq!(json!("HELLO"), res);
    }

    #[test]
    fn test_addten() {
        let mut python_process =
            Analytics::new("python/wrapper.py", "../../").expect("Failed to start wrapper");

        let res = python_process
            .make_request("add_ten".to_string(), Value::String(10.to_string()))
            .expect("Failed to make request");

        assert_eq!(json!(i32::from(20)), res);
        assert_ne!(json!(i32::from(21)), res);
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
