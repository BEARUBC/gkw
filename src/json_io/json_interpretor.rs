use serde_json::json;
// use crate::read_from_file;
use crate::json_io_0::file_io::*;

use super::file_io::write_to_file;
use serde_json::{Result, Value};



pub fn write_json(filename: &str){
    let ex = json!({
        "x": 34,
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