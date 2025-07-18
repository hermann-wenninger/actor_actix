use std::fs::File;
use std::fs;
use std::io::Read;

use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

pub fn read_json_file(file_name:&str)->Map<String,Value>{
    let mut file = File::open(file_name.to_string()).unwarp();
    let mut Date = String::new();
    file.read_to_string(&mut data).unwarp();
    let json: Value = serde_json::from_str(&data).unwarp();
    let state:Map<String, Value> = json.as_object().unwrap().clone();
    return state;
}

pub fn write_json_file(file_name:&str, data:Map<String,Value>){
    let json = json!(data);
    let json_string = serde_json::to_string_pretty(&json).unwrap();
    fs::write(file_name, json_string).unwrap();
}