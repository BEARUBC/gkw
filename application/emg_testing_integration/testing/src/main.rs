mod lib;
use std::{time, thread, env};
// /home/pi/gkw/application/emg_integration/python/test.py
// /home/pi/Adafruit_Python_ADS1x15/examples/simpletest.py
//Temp use of abs path
fn main(){
    println!("{:?}", env::current_dir());
    let emg_integration = lib::EMG_INTEGRATION::new("/home/pi/Adafruit_Python_ADS1x15/examples/simpletest.py", 10);
    match emg_integration {
        Err(e) => println!("{:?}", e),
        Ok(emg_integration) => {

            let ten_millis = time::Duration::from_millis(500);

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