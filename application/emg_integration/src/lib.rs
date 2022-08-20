use std::{thread, time};
use std::thread::JoinHandle;
use std::process::{Command, Stdio, Child};
use std::io::{BufReader, BufRead, ErrorKind};
use std::io::Error as StdError;
use std::collections::VecDeque;
use std::vec::Vec;
use std::sync::{Arc, Mutex};

pub struct EMG_INTEGRATION {
    maxRequestSize: u16,
    data: Arc<Mutex<VecDeque<u32>>>,
    read_thread: JoinHandle<()>,
    child: Child
}

impl EMG_INTEGRATION{
    //Description: Spawns a child process that reads value from EMG device, and stores 
    //Parameters: child_process - name of the child process
    //Returns: Analytics struct 
    pub fn new(emg_cmd: &str, requestSize: u16) -> Result<EMG_INTEGRATION, StdError> {

        let maxRequestSize = requestSize;
        
        let mut child = Command::new("python3")
                                .args([emg_cmd])
                                .stdout(Stdio::piped())
                                .spawn()?;

        let pipe = child.stdout.take().expect("Failed to get stdout");
        let data = Arc::new( Mutex::new( VecDeque::with_capacity(maxRequestSize.into()) ) );

        let data_clone = data.clone();

        return Ok( 
            EMG_INTEGRATION{
                maxRequestSize: maxRequestSize,
                child: child,
                data: data,
                read_thread: thread::spawn(move || {
        
                    let mut buf_reader = BufReader::new(pipe);
                    
                    loop {
                        let mut data_str = String::new();

                        buf_reader.read_line(&mut data_str).unwrap();
                        data_str.pop();
                        data_clone.lock().unwrap().push_back( data_str.parse::<u32>().unwrap() );
                    
                        if data_clone.lock().unwrap().len() > maxRequestSize.into() {
                            data_clone.lock().unwrap().pop_front();
                        }
                    }
                }),
            },
        );            
    }

    pub fn get_data_queue(&self, data_num: u32) -> Result<Vec<u32>, StdError> {
        let mut ret_data: Vec<u32> = Vec::new();

        if data_num < 0 || data_num > self.maxRequestSize.into() {
            return Err(StdError::new(ErrorKind::Other, "data_num must be greater than or equal to 0, less than requestSize"));
        }

        let mut read_data = self.data.lock().unwrap().clone();

        for _ in 0..std::cmp::min(data_num, read_data.len() as u32) {
            let data = read_data.pop_front();
            match data {
                None => break,
        
                Some(data) => {
                    ret_data.push(data);
                },
            }
        }
        return Ok(ret_data);
    }

    pub fn kill_emg(&mut self) -> Result<(), StdError> {
        self.child.kill()
    }
}

impl Drop for EMG_INTEGRATION {
    fn drop(&mut self) {
            println!("DROPPING EMG");
            // self.child.kill();
            (self).kill_emg();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_data_queue() {
        let emg_integration = EMG_INTEGRATION::new("python/test.py", 10);
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
                            assert_eq!(results[i], prev+1);
                            prev = results[i];
                        }

                        return;
                    }
                }
            }
        }

        assert_eq!(false, true);
    }
}