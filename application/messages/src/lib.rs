use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;

use anyhow::Result;
use uuid::Uuid;

pub struct Custom_log{
    id: Uuid,
    file: File
}

impl Custom_log{

    pub fn new() -> Result<Custom_log>{
        let id = Uuid::new_v4();
        let id_string = id.to_hyphenated().to_string();
        let mut word = "logs/".to_owned();
        word.push_str(&id_string);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(word + ".txt")?;
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