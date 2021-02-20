use serde_json::json;
use crate::read_from_file;

use super::file_io::write_to_file;
use serde_json::{Result, Value};

pub fn deserial_json() -> Result<()>{
    let json_object = read_from_file("input.json");
    
    let j: Value = serde_json::from_str(&json_object)?;

    write_to_file(&j.to_string(), "output.json");

    Ok(())
}

// pub fn testJson() {
//     let x = deserial_json();
    
//     write_to_file(&x.to_string(), "output.json")
// }

pub fn instance() {
    let ex = json!({
        "x": 3,
        "y": "a string",
        "z": [3,4,5],
    });

    let json_object = json!(read_from_file("input.json"));

    let j = serde_json::to_string(&json_object);

    //println!("{}", test.to_string());
    //println!("{}", ex.to_string());
}

pub fn write_json(filename: &str){
    let ex = json!({
        "x": 3,
        "y": "a string",
        "z": [3,4,5],
    });

    write_to_file(&ex.to_string(), filename)
}

// use serde_json::{Result,Value};

// fn instance() -> Result<()> {
//     let ex = r#"{
//         "x": 2
//         "y": 4
//         "z": [-3,4,5]
//     }"#;

//     let val: Value = serde_json::from_str(data)?;

//     println!("x = {}, y = {}, z = {}", val[x],val[y],val[z]);

//     Ok(())
// }