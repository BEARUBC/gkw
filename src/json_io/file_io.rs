use std::fs::File;
use std::io::prelude::*;

pub fn read_from_file(file_name: &str) -> String {
    let mut file = File::open("./py_io/".to_owned() + file_name).expect("Unable to open file!");
    let mut json_content = String::new();
    file.read_to_string(&mut json_content).expect("Unable to read to file!");

    json_content
}

pub fn write_to_file(str: &String, file_name: &str) {
    let mut file = File::create("./py_io/".to_owned() + file_name).expect("Unable to create file!");
    file.write_all(str.as_bytes()).expect("Unable to write to file!");
}
