use std::io::{Write, BufReader, BufRead};
use std::{thread, time};
use std::thread::JoinHandle;
use std::process::{Command, Stdio, Child, ChildStdin};
use std::collections::{HashMap};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use serde_json::value::Value;
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
    data: String
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
        let child_process_res = Command::new("python")
                .args([python_process])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn();

        // ensure child process starts correctly
        let mut child_process = if let Ok(child_process) = child_process_res{
            child_process
        } else {
            return Err(gkwError::new(code::other, Some("Failed to start child process")));
        };

        //get stdin and stdout and verify it works
        let stdin_res = child_process.stdin.take();
        let stdin = if let Some(stdin) = stdin_res{
            stdin
        } else {
            return Err(gkwError::new(code::other, Some("Failed to get stdin")));
        };

        let stdout_res = child_process.stdout.take();
        let stdout = if let Some(stdout) = stdout_res{
            stdout
        } else {
            return Err(gkwError::new(code::other, Some("Failed to get stdout")));
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
                if let Err(e) = check_read{
                    //continue if it fails, to avoid an infinite loop it will eventually timeout
                    continue;
                }

                //check if there was a response or if there was nothing
                if resp_string!="" {
                    //turn the string from Python into a Response struct and verify it works
                    let response_packet_res:Result<Response, serde_json::Error> = serde_json::from_str(&resp_string);
                    let response_packet = if let Ok(response_packet)=response_packet_res{
                        response_packet
                    } else{
                        //continue if it fails
                        continue;
                    };

                    //get access to the map and verify it works
                    let locked_map_clone_res = storage_clone.lock(); 
                    let mut locked_map_clone = if let Ok(locked_map_clone) = locked_map_clone_res{
                        locked_map_clone
                    }
                    else{
                        //continue if it fails
                        continue;
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
    pub fn make_request(&mut self, func: String, parameters: Value) -> gkwResult<String> {
        // make the request object
        let my_uuid = Uuid::new_v4();
        let request_packet = Request{
            request_id: my_uuid,
            request_type: func,
            params: parameters
        };

        let json_str_res = serde_json::to_string(&request_packet);
        let json_string = if let Ok(json_string) = json_str_res {
            json_string + "\n"
        } else {
            // Error case, return error
            return Err(gkwError::new(code::other, Some("Unable to stringify request.")));
        };

        // make space in map
        {
            let locked_map_res = self.response_holder.lock();
            let mut locked_map = if let Ok(locked_map) = locked_map_res {
                locked_map
            } else {
                return Err(gkwError::new(code::other, Some("Unable to make space in map.")));
            };
            locked_map.insert(my_uuid, None);
        }

        println!("Sending {}", json_string);
        // send string over
        let result = self.stdin.write_all(json_string.as_bytes());
        if let Err(e) = result {
            return Err(gkwError::new(code::other, Some("Failed to write to child's stdin.")));
        }
        println!("Sent Data");

        // wait for child process to finish processing
        loop{
            let locked_map_res = self.response_holder.lock();
            let locked_map = if let Ok(locked_map) = locked_map_res {
                locked_map
            } else {
                return Err(gkwError::new(code::other, Some("Cannot lock map.")));
            };
            let val_res = locked_map.get(&my_uuid);
            let val = if let Some(val) = val_res {
                val
            } else {
                return Err(gkwError::new(code::other, Some("Failed to get value in map")));
            };
            if let Some(res) = val{
                return Ok(res.data.clone());
            }
            let ten_millis = time::Duration::from_millis(10);
            thread::sleep(ten_millis);
        }
        
    }    
    // TODO:
    // - Change the return type of new and make request to Result
    // - Make places with expect or unwrap return error
    // - Change the return statment to wrap it in the proper type
    // - When checking response continue instead of throwing an error and eventually timeout
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize() {
        let python_process_res = Analytics::new("./python/wrapper.py");
        let mut python_process = if let Ok(python_process) = python_process_res{
            python_process
        } else {
            panic!("Failed to start wrapper");
        };

        let result = python_process.make_request("capitalize".to_string(), Value::String("hello".to_string()));
        let res = if let Ok(res) = result {
            res
        } else {
            println!("Failed, error: \"{:?}\"", result);
            "failed".to_string()
        };

        assert_eq!("HELLO", res);
    }

    #[test]
    fn test_addten() {
        let python_process_res = Analytics::new("./python/wrapper.py");
        let mut python_process = if let Ok(python_process) = python_process_res{
            python_process
        } else {
            panic!("Failed to start wrapper");
        };

        let result = python_process.make_request("add_ten".to_string(), Value::String(10.to_string()));
        let res = if let Ok(res) = result {
            res
        } else {
            println!("Failed, error: \"{:?}\"", result);
            "failed".to_string()
        };

        assert_eq!(20.to_string(), res);
        assert_ne!(21.to_string(), res);
    }


    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

