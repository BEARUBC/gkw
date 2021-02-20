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
    #[test]
    fn it_works() -> () {
        assert_eq!(2u8, 2u8);
    }
}
