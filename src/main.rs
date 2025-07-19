mod doit;
mod state;
use doit::structs::done::Done;
use doit::structs::pending::Pending;

use doit::ItemTypes;
use doit::doit_factory;
use doit::structs::traits::create::Create;

use state::{read_json_file,write_json_file};
use serde_json::{Map,json};
use serde_json::value::Value;

fn main()  {

let args = std::env::args().collect::<Vec<String>>();
if args.len() < 3 {
    eprintln!("Usage: {} <status> <title>", args[0]);
    std::process::exit(1);
}
let status: &String = &args[1];
let titel: &String = &args[2];

let mut state: Map<String,Value> = read_json_file(&String::from("./jsonfile.json"));

state.insert(titel.to_string(), json!(status));
 write_json_file("./jsonfile.json",  state);












 
let doit_item:Result<ItemTypes, &'static str> = doit_factory("pending","find work");
match doit_item.unwrap(){

    ItemTypes::Pending(item)=> item.create(&item.super_struct.title),
    ItemTypes::Done(item)=>println!("Ist n done item with the value::: {}",item.super_struct.title),
}

let done: Done = Done::new("programming");
 println!("{}", done.super_struct.title);
 println!("{}", done.super_struct.status);
 let pending: Pending = Pending::new("teaching");
 println!("{}", pending.super_struct.title);
 println!("{}", pending.super_struct.status);
}
