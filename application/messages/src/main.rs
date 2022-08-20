use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;

use gkw_utils::Result as gkwResult;
use gkw_utils::Error as gkwError;
use gkw_utils::ErrorCode as code;
use messages::Custom_log;
use uuid::Uuid;

// fn main() -> gkwResult<()> {
//     let id = Uuid::new_v4();
//     let id_string = id.to_hyphenated().to_string();
//     let mut word = "logs/".to_owned();
//     word.push_str(&id_string);
//     let mut file = match OpenOptions::new()
//             .read(true)
//             .write(true)
//             .create(true)
//             .open(word + ".txt"){
//                 Ok(file) => file,
//                 Err(e) => {
//                     return Err(gkwError::new(code::other, Some(format!("Unable to make space in map. Error: {}", e))));
//                 }
//             };
            
//     file.write_all(b"Horld2! \n");
//     file.write_all(b"op 222222222");
//     Ok(())
// }

fn main(){
    let log_res = Custom_log::new();
    let mut log = if let Ok(log) = log_res{
        log
    } else {
        panic!("Failed to start log");
    };

    log.write_log(b"testing testing 1 2 3");
    log.write_log(b"\n next");
}
