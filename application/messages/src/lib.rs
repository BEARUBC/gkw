use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;

use gkw_utils::Result as gkwResult;
use gkw_utils::Error as gkwError;
use gkw_utils::ErrorCode as code;
use uuid::Uuid;

pub struct Custom_log{
    id: Uuid,
    file: File
}

impl Custom_log{

    pub fn new() -> gkwResult<Custom_log>{
        let id = Uuid::new_v4();
        let id_string = id.to_hyphenated().to_string();
        let mut word = "logs/".to_owned();
        word.push_str(&id_string);
        let mut file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(word + ".txt"){
                Ok(file) => file,
                Err(e) => {
                    return Err(gkwError::new(code::other, Some(format!("Unable to make space in map. Error: {}", e))));
                }
            };
        return Ok(
            Custom_log{
                id: id,
                file: file
        });
    }

    pub fn write_log(&mut self, text: &[u8]){
        self.file.write_all(text);
    }
}