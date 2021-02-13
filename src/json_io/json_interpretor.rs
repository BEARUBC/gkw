use serde_json::json;

pub fn instance() {
    let ex = json!({
        "x": 3,
        "y": "a string",
        "z": [3,4,5],
    });

    println!("{}", ex.to_string());
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