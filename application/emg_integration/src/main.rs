mod lib;
use std::{time, thread};


fn main(){
    let emg_integration = lib::EMG_INTEGRATION::new("application/emg_integration/python/test.py", 10);
    match emg_integration {
        Err(e) => println!("{:?}", e),
        Ok(emg_integration) => {

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