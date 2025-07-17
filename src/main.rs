mod doit;
use doit::structs::done::Done;
use doit::structs::pending::Pending;

use doit::ItemTypes;
use doit::doit_factory;
use doit::structs::traits::create::Create;

fn main()  {

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
