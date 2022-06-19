
use std::io::{Write, Read};
use std::process::{Command, Stdio, Child, ChildStdin, ChildStdout};
mod library;
use std::{time, thread};


fn main(){
    let emg_integration = library::EMG_INTEGRATION::new("../python/test.py", 10);
    match emg_integration {
        Err(e) => println!("{:?}", e),
        Ok(mut emg_integration) => {

            let ten_millis = time::Duration::from_millis(100);

            thread::sleep(ten_millis);
            let x = emg_integration.get_data_queue(9);
            match x {
                Err(e) => println!("{:?}", e),
                Ok(x) => {
                    println!("got data is: {:?}", x);

                
                }
            }

            thread::sleep(ten_millis);
            let x = emg_integration.get_data_queue(9);
            match x {
                Err(e) => println!("{:?}", e),
                Ok(x) => {
                    println!("got data is: {:?}", x);

                }
            }

            thread::sleep(ten_millis);
            let x = emg_integration.get_data_queue(9);
            match x {
                Err(e) => println!("{:?}", e),
                Ok(x) => {
                    println!("got data is: {:?}", x);

                }
            }
        }
    }
    println!("DONE");
}