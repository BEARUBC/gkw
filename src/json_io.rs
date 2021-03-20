use std::{
    fs,
    fs::File,
    fs::OpenOptions,
    io::{
        prelude::*,
        ErrorKind
    },
};
use serde_json::{
    Result,
    Value,
    json,
};


pub fn read_from_output() -> std::io::Result<Value> {
    let mut string_buffer = String::new();

    match File::open(format!("./py_io/output.json")) {
        Ok(mut file) => {
            match file.read_to_string(&mut string_buffer) {
                Ok(_) => {
                    match serde_json::from_str(string_buffer.as_str()) {
                        Ok(value) => { return Ok(value); },
                        Err(err) => { return Err(std::io::Error::new(ErrorKind::Other, "unable to construct json from file")); },
                    }
                },
                Err(err) => { return Err(err); },
            };
        },
        Err(err) => { return Err(err); },
    };
}

pub fn write_to_input(json: Value) -> std::io::Result<()> {
    let json_string = json.to_string();
    let json_bytes = json_string.as_bytes();

    let mut file = OpenOptions::new().read(true).write(true).create(true).truncate(true).open("./py_io/input.json").unwrap();
        
    match file.write_all(json_bytes){
        Ok(()) => { return Ok(()); },
        Err(err) => { return Err(err); }
    }

    // match File::create("./py_io/input.json").unwrap() {
    //     Ok(mut file) => {
    //         match file.write_all(json_bytes) {
    //             Ok(()) => { return Ok(()); },
    //             Err(err) => { return Err(err); }
    //         }
    //     },
    //     Err(err) => { return Err(err); },
    // };
}

#[cfg(test)]
mod json_io_test {
    use serde_json::{Result, Value, json,};
    use super::*;
    use serde::{Deserialize, Serialize};

    #[test]
    fn simple_write() -> () {
        let json = json!({
            "x": 34
        });

        let result = write_to_input(json);

        match result {
            Ok(()) => (),
            Err(e) => { panic!(); },
        }
    }

    #[test]
    fn complex_write() -> () {
        let json = json!({
            "x": 34,
            "obj": {
                "asdf": true,
            },
        });

        let result = write_to_input(json);

        match result {
            Ok(()) => (),
            Err(e) => { panic!(); },
        }
    }

    #[test]
    fn simple_read() -> () {
        match read_from_output() {
            Ok(json) => (),
            Err(err) => { panic!(); }
        };
    }

    #[derive(Serialize, Deserialize)]
    struct Emg{
        strength: String,
        other_fields: String
    }
    #[derive(Serialize, Deserialize)]
    struct Test {
        x: Emg,
        other_projects: String,
    }

    #[test]
    fn test_struct() -> () {
        let data= r#"
        {
            "x": {
                "strength": "strong",
                "other_fields": "other"
            },
            "other_projects": "projects"
        }"#;
       
        let test: Test = serde_json::from_str(data).expect("Json error");
        
        let e= Emg{
            strength: "strong".to_string(),
            other_fields: "other".to_string()
        }; 

        let oth_proj = "projects".to_string();

        assert_eq!(test.x.strength, e.strength);
        assert_eq!(test.x.other_fields, e.other_fields);
        assert_eq!(test.other_projects, oth_proj);        
    }
}
