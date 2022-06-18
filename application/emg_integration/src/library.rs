use std::{thread, time};
use std::thread::JoinHandle;
use std::process::{Command, Stdio, Child, ChildStdin, ChildStdout};
use std::io::{Write, Read, BufReader, BufRead, ErrorKind};
use std::io::Error as StdError;
use std::vec::Vec;
use std::sync::{Arc, Mutex};

use gkw_utils;

pub struct EMG_INTEGRATION {
    pub data: Arc<Mutex<Vec<u32>>>,
    pub read_thread: JoinHandle<()>,
    child: Child
}

impl EMG_INTEGRATION{
    pub fn new(emg_cmd: &str) -> Result<EMG_INTEGRATION, StdError> {
        
        let mut child = Command::new("python")
                                .args([emg_cmd])
                                .stdout(Stdio::piped())
                                .spawn()?;

        let pipe = child.stdout.take().expect("Failed to get stdout");
        let data = Arc::new( Mutex::new( Vec::new() ) );

        let mut data_clone = data.clone();

        return Ok( 
            EMG_INTEGRATION{
                child: child,
                data: data,
                read_thread: thread::spawn(move || {
        
                    let mut buf_reader = BufReader::new(pipe);
                    
                    loop {
                        let mut data_str = String::new();

                        buf_reader.read_line(&mut data_str).unwrap();
                        data_str.pop();
                        data_clone.lock().unwrap().push( data_str.parse::<u32>().unwrap() );
                        
                        let data_check = data_clone.lock().unwrap();
                        
                        let data_from_check = data_check.get(data_check.len() - 1);  
                    }
                }),
            },
        );            
    }

    pub fn get_data_queue(&self, data_num: u32) -> Result<Vec<u32>, StdError> {
        let mut ret_data: Vec<u32> = Vec::new();

        if data_num < 0 {
            return Err(StdError::new(ErrorKind::Other, "data_num must be greater than or equal to 0"));
        }

        let mut read_data = self.data.lock().unwrap().clone();

        for _ in 0..std::cmp::min(data_num, read_data.len() as u32) {
            let data = read_data.pop();
            match data {
                None => break,
        
                Some(data) => {
                    ret_data.push(data);
                },
            }
        }
        return Ok(ret_data);
    }

    pub fn kill_emg(mut self) -> Result<(), StdError> {
        self.child.kill()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_data_queue() {
        let emg_integration = EMG_INTEGRATION::new("python/test.py");
        match emg_integration {
            Err(e) => println!("ERROR IS {:?}", e),
            Ok(emg_integration) => {
    
                let ten_millis = time::Duration::from_millis(1000);
    
                thread::sleep(ten_millis);
                let results = emg_integration.get_data_queue(9);
                match results {
                    Err(e) => panic!("ERROR IS {:?}", e),
                    Ok(results) => {
                        println!("got data is: {:?}", results);

                        assert_eq!(results.len(), 9);
                        let mut prev = results[0];

                        for i in 1..results.len() - 1 {
                            assert_eq!(results[i], prev-1);
                            prev = results[i];
                        }

                        emg_integration.kill_emg();
                        return;
                    }
                }
            }
        }

        assert_eq!(false, true);
    }
}