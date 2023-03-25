mod lib;
use std::{time, thread, env};
use std::time::{Duration, Instant};
use std::fs::OpenOptions;
use serde_derive::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
struct Data {
    data: Vec<f32>,
    timez: u128,
}

fn main(){
    // let mut file = OpenOptions::new()
    // .read(true)
    // .append(true)
    // .create(true)
    // .open("foo.txt")
    // .expect("cannot open file");
  

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
                    let end = start.elapsed().as_millis();
                    println!("got data is: {:?}. Time is {}", x, end);

                    let d1 = Data{
                    data: x,
                    timez: start.elapsed().as_millis(),
                    };
                    
                    std::fs::write(
                        "foo.txt",
                        serde_json::to_string_pretty(&d1).unwrap(),
                    ).expect("Write failed");

                }
            }

            thread::sleep(ten_millis);
            let x = emg_integration.get_data_queue(9);
            match x {
                Err(e) => println!("{:?}", e),
                Ok(x) => {
                    let end = start.elapsed().as_millis();
                    println!("got data is: {:?}. Time is {}", x, end);

                    let d2 = Data{
                    data: x,
                    timez: start.elapsed().as_millis(),
                    };

                    std::fs::write(
                        "foo.txt",
                        serde_json::to_string_pretty(&d2).unwrap(),
                    ).expect("Write failed");              }
            }

            thread::sleep(ten_millis);
            let x = emg_integration.get_data_queue(9);
            match x {
                Err(e) => println!("{:?}", e),
                Ok(x) => {
                    let end = start.elapsed().as_millis();
                    println!("got data is: {:?}. Time is {}", x, end);

                    let d3 = Data{
                    data: x,
                    timez: start.elapsed().as_millis(),
                    };

                    std::fs::write(
                        "foo.txt",
                        serde_json::to_string_pretty(&d3).unwrap(),
                    ).expect("Write failed");         }
            }
        }
    }
    println!("DONE");

}