use std::{thread, time};
use std::thread::JoinHandle;
use std::process::{Command, Stdio, Child, ChildStdin, ChildStdout};
use std::io::{Write, Read, BufReader, BufRead, ErrorKind};
use std::io::Error as StdError;
use std::vec::Vec;
use std::sync::{Arc, Mutex};

use gkw_utils;

// #[derive(Clone)]
pub struct EMG_INTEGRATION {
    //pipe: std::process::ChildStdout,
    pub data: Arc<Mutex<Vec<u8>>>,
    pub read_thread: JoinHandle<()>

}

// trait Get_Data{
    //fn get_data(&self, data_num: u8) -> Result<Vec<[u8; 8]>, StdError>;
// }


impl EMG_INTEGRATION{
    pub fn new() -> Result<EMG_INTEGRATION, StdError> {
        
        let mut child = Command::new("python")
                                .args(["../python/test.py"])
                                .stdout(Stdio::piped())
                                .spawn()?;

        let pipe = child.stdout.take().expect("Failed to get stdout");
        let data = Arc::new( Mutex::new( Vec::new() ) );

        let mut data_clone = data.clone();

        return Ok( 
            EMG_INTEGRATION{
                data: data,
                read_thread: thread::spawn(move || {
        
                    let mut buf_reader = BufReader::new(pipe);
                    
                    loop {
                        // println!("loop print");

                        let mut data_str = String::new();

                        buf_reader.read_line(&mut data_str).unwrap();

                        // print!("before: {}end\n", data_str);

                        data_str.pop();

                        // print!("after: {}end\n", data_str);
                    
                        data_clone.lock().unwrap().push( data_str.parse::<u8>().unwrap() );
                        
                        let data_check = data_clone.lock().unwrap();
                        
                        let data_from_check = data_check.get(data_check.len() - 1);

                        

                        
                    }
        
                }),
            },
        );            
    }
    // pub fn init_thread(&'static self) -> 
    // {
    //     let m_thread = thread::spawn(move || {
    //         let mut child = Command::new("../emg")
    //                         .stdout(Stdio::piped())
    //                         .stdin(Stdio::piped())
    //                         .spawn()?;

    //         let pipe = child.stdout.take().expect("Failed to get stdout");

    //         let mut buf_reader = BufReader::new(pipe);
            
    //         loop {
    //             let mut data_str = String::new();
    //             buf_reader.read_line(&mut data_str).unwrap();
                
    //             let mut data_clone = self.data.lock().unwrap(); 

    //             data_clone.push( str_to_array(data_str.clone()) );

    //             println!("{:?}", data_str);
    //         }

    //     });
    // }


    pub fn get_data(&self, data_num: u8) -> Result<Vec<u8>, StdError> {
        // let mut f = BufReader::new(self.pipe);
        let mut ret_data: Vec<u8> = Vec::new();

        if data_num < 0 {
            return Err(StdError::new(ErrorKind::Other, "data_num must be greater than or equal to 0"));
        }

        let mut read_data = self.data.lock().unwrap().clone();

        for _ in 0..std::cmp::min(data_num, read_data.len() as u8) {
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
}




// fn str_to_array(s: String) -> [u8; 8]{
//     let mut data: [u8; 8] = [0; 8];
//     let mut counter = 0;
//     let zero_ASCII: u8 = ('0') as u8;
//     //println!("{}", s.chars());
//     for char in s.chars(){
//         if char != '\n' {
//             break;
//         }
//         //println!("{}, {}, {}", char as u32, zero_ASCII, char as u8 - zero_ASCII);
//         let d = char as u8 - zero_ASCII;
//         data[counter] = d;
//         counter += 1;
//     }
//     return data;
// }



//  impl Get_Data for EMG_INTEGRATION{
    
//  }