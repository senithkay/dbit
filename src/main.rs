use std::env;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    _changes: String,

}

#[derive(Debug, Deserialize, Serialize)]
struct Meta {
    _init_date: String,
    _db_type: String,

}

#[derive(Debug, Deserialize, Serialize)]
struct JFile {
    
    _meta: Meta,
    _data: Data,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_lv1 = &args[1];
    match args_lv1.as_str() {
        "init" => {
            let json_string = r#"
            {
                "_meta": {
                    "_init_date": "",
                    "_db_type": ""
                },
                "_data" : {
                    "_changes" : ""
                }
            }
        "#;
            let parsed_file: JFile = serde_json::from_str(json_string).expect("Failed to parse JSON");

       
            let json_string = serde_json::to_string(&parsed_file).expect("Failed to serialize to JSON");
            println!("{}", json_string);
            let mut file = File::create("output.json").expect("Failed to create file");
            file.write_all(json_string.as_bytes()).expect("Failed to write to file");
            
        }
        

        _ => {
            println!("Unknown argument");
        }
    }
   
}


