mod lib;
use std::{time, thread, env};
use std::time::{Duration, Instant};

fn main(){
    let start = Instant::now(); 
    println!("{:?}", env::current_dir());
    //let emg_integration = lib::EMG_INTEGRATION::new("C:/Users/Ray Ho/Documents/UBC BIONICS/gkw/application/emg_integration/python/test.py", 100);
    let emg_integration = lib::EMG_INTEGRATION::new("../python/emg_testing.py", 10);
    match emg_integration {
        Err(e) => println!("{:?}", e),
        Ok(emg_integration) => {

            let ten_millis = time::Duration::from_millis(500);

            thread::sleep(ten_millis);
            let x = emg_integration.get_data_queue(9);
            match x {
                Err(e) => println!("{:?}", e),
                Ok(x) => {
                    println!("got data is: {:?}. Time is {}", x, start.elapsed().as_millis());
                }
            }

            thread::sleep(ten_millis);
            let x = emg_integration.get_data_queue(9);
            match x {
                Err(e) => println!("{:?}", e),
                Ok(x) => {
                    println!("got data is: {:?}. Time is {}", x, start.elapsed().as_millis());                }
            }

            thread::sleep(ten_millis);
            let x = emg_integration.get_data_queue(9);
            match x {
                Err(e) => println!("{:?}", e),
                Ok(x) => {
                    println!("got data is: {:?}. Time is {}", x, start.elapsed().as_millis());                }
            }
        }
    }
    println!("DONE");

}