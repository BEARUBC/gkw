use std::{
    fs,
    fs::File,
};
use std::io::prelude::*;

pub fn read_from_output() -> std::io::Result<String> {
    let mut string_buffer = String::new();

    match File::open(format!("./py_io/output.json")) {
        Ok(mut file) => {
            match file.read_to_string(&mut string_buffer) {
                Ok(_) => { return Ok(string_buffer); },
                Err(err) => { return Err(err); },
            };
        },
        Err(err) => { return Err(err); },
    };
}

pub fn write_to_input(content: &[u8]) -> std::io::Result<()> {
    match File::open("./py_io/input.json") {
        Ok(mut file) => {
            match file.write(content) {
                Ok(_) => { return Ok(()); },
                Err(err) => { return Err(err); },
            }
        },
        Err(err) => { return Err(err); },
    };
}

#[cfg(test)]
mod json_io_test {
    use serde_json::{Result, Value, json};
    use super::write_to_input;

    #[test]
    fn simple_write() -> () {
        let ex = json!({
            "x": 34,
            "y": "a string",
            "z": [3,4,5],
        });

        let j = ex.to_string();

        let x = j.as_bytes();

        let result = write_to_input(x);

        match result{
            Ok(()) => {},
            Err(e) => {panic!();},
        }
    }

    // pub fn deserial_json() -> Result<()>{
    //     let json_object = read_from_file("input.json");
        
    //     let j: Value = serde_json::from_str(&json_object)?;
    
    //     write_to_file(&j.to_string(), "output.json");
    
    //     Ok(())
    // }
    
    // pub fn testJson() {
    //     let x = deserial_json();
        
    //     write_to_file(&x.to_string(), "output.json")
    // }
    
    // pub fn instance() {
    //     let ex = json!({
    //         "x": 34,
    //         "y": "a string",
    //         "z": [3,4,5],
    //     });
    
    //     let json_object = json!(read_from_file("input.json"));
    
    //     let j = serde_json::to_string(&json_object);
    
    //     //println!("{}", test.to_string());
    //     //println!("{}", ex.to_string());
    // }
}
