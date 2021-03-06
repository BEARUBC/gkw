use std::io::{Write, BufReader, BufRead};
use std::{thread, time, format};
use std::thread::JoinHandle;
use std::process::{Command, Stdio, Child, ChildStdin};
use std::collections::{HashMap};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use serde_json::value::Value;
pub use serde_json::json;
use std::sync::{Arc, Mutex};
use gkw_utils::Result as gkwResult;
use gkw_utils::Error as gkwError;
use gkw_utils::ErrorCode as code;

#[derive(Serialize, Deserialize, Debug)] 
pub struct Request {
    request_id: Uuid,
    request_type: String,
    params: Value
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    request_id: Uuid,
    valid_bit: i8,
    data: Value
}

pub struct Analytics{
    child_process: Child,
    stdin: ChildStdin,
    read_thread: JoinHandle<()>,
    response_holder: Arc<Mutex<HashMap<Uuid, Option<Response>>>>
}

impl Analytics {
    //Description: Begins the process of getting a response from the Python code. Starts the child
    //process then loops until it gets a response. Once it recieves a response it puts it in storage
    //Parameters: child_process - name of the child process
    //Returns: Analytics struct 
    pub fn new(python_process: &str) -> gkwResult<Analytics>{
        // start child process
        let mut child_process = match Command::new("python3")
                .args([python_process])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn() {
            Ok(child_process) => child_process,
            Err(e) => {
                return Err(gkwError::new(code::other, Some(format!("Failed to start child process. Error: {}", e))));
            }
        };

        //get stdin and stdout and verify it works
        let stdin = match child_process.stdin.take() {
            Some(stdin) => stdin,
            None => {
                return Err(gkwError::new(code::other, Some("Failed to get stdin")));
            }
        };
        let stdout = match child_process.stdout.take() {
            Some(stdin) => stdin,
            None => {
                return Err(gkwError::new(code::other, Some("Failed to get stdout")));
            }
        };

        //create a map and a map clone for the data
        let storage = Arc::new(Mutex::new(HashMap::new()));
        let storage_clone = Arc::clone(&storage);

        //Begin the read thread
        let read_thread = thread::spawn(move || {
            let mut f = BufReader::new(stdout);
            loop{
                let mut resp_string = String::new();

                //read from the stdout using Bufreader and verify it works
                let check_read = f.read_line(&mut resp_string);
                if let Err(_e) = check_read{
                    //continue if it fails, to avoid an infinite loop it will eventually timeout
                    continue;
                }

                //check if there was a response or if there was nothing
                if resp_string!="" {
                    //turn the string from Python into a Response struct and verify it works
                    let response_packet: Response = match serde_json::from_str(&resp_string) {
                        Ok(response_packet) => response_packet,
                        Err(_e) => {
                            //continue if it fails
                            continue;
                        }
                    };

                    //get access to the map and verify it works
                    let mut locked_map_clone = match storage_clone.lock() {
                        Ok(locked_map_clone) => locked_map_clone,
                        Err(_e) => {
                            //continue if it fails
                            continue;
                        }
                    };
                    
                    //insert the response into the map
                    locked_map_clone.insert(response_packet.request_id, Option::Some(response_packet));
                }
            }
        });

        return Ok(
            Analytics {
            child_process: child_process,
            stdin: stdin,
            read_thread: read_thread,
            response_holder: storage
        });
    }

    //Description: Takes a function request and parameters and sends instructions to Analytics code that
    //fulfills the request. After sending the request it will loop until it finds the reponse in storage
    //then returns the response
    //Parameters: func - string that specifies the function
    //            parameters - map that holds each parameter name and value
    //Returns: String version of the response
    pub fn make_request(&mut self, func: String, parameters: Value) -> gkwResult<Value> {
        // make the request object
        let my_uuid = Uuid::new_v4();
        let request_packet = Request{
            request_id: my_uuid,
            request_type: func,
            params: parameters
        };

        let json_string = match serde_json::to_string(&request_packet) {
            Ok(json_string) => json_string + "\n",
            Err(e) => {
                // Error case, return error
                return Err(gkwError::new(code::other, Some(format!("Unable to stringify request. Error: {}", e))));
            }
        };

        // make space in map
        {
            let mut locked_map = match self.response_holder.lock() {
                Ok(locked_map) => locked_map,
                Err(e) => {
                    return Err(gkwError::new(code::other, Some(format!("Unable to make space in map. Error: {}", e))));
                }
            };
            locked_map.insert(my_uuid, None);
        }

        println!("Sending {}", json_string);
        // send string over
        let result = self.stdin.write_all(json_string.as_bytes());
        if let Err(e) = result {
            return Err(gkwError::new(code::other, Some(format!("Failed to write to child's stdin. Error: {}", e))));
        }
        println!("Sent Data");

        // wait for child process to finish processing
        loop{
            let locked_map = match self.response_holder.lock() {
                Ok(locked_map) => locked_map,
                Err(e) => {
                    return Err(gkwError::new(code::other, Some(format!("Cannot lock map. Error: {}", e))));
                }
            };
            
            let val = match locked_map.get(&my_uuid) {
                Some(val) => val,
                None => {
                    return Err(gkwError::new(code::other, Some("Failed to get value in map")));
                }
            };
            if let Some(res) = val{
                return Ok(res.data.clone());
            }
            let ten_millis = time::Duration::from_millis(10);
            thread::sleep(ten_millis);
        }
        
    }    
    // TODO:
    // - When checking response eventually timeout
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize() {
        let mut python_process = Analytics::new("./python/wrapper.py").expect("Failed to start wrapper");
        let res = python_process
            .make_request("capitalize".to_string(), Value::String("hello".to_string())).expect("Failed to make request");
        assert_eq!(json!("HELLO"), res);
    }

    #[test]
    fn test_addten() {
        let mut python_process = Analytics::new("./python/wrapper.py").expect("Failed to start wrapper");

        let res = python_process
            .make_request("add_ten".to_string(), Value::String(10.to_string())).expect("Failed to make request");

        assert_eq!(json!(i32::from(20)), res);
        assert_ne!(json!(i32::from(21)), res);
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

