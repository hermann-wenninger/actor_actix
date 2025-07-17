pub mod structs;

use structs::done::Done;
use structs::pending::Pending;

pub enum ItemTypes{
    Pending(Pending),
    Done(Done),
}

pub fn doit_factory(item_type:&str, item_titele:&str)->Result<ItemTypes,&'static str>{
if item_type == "pending"{
    let pend_item = Pending::new(item_titele);
    Ok(ItemTypes::Pending(pend_item))
}
else if item_type == "done"{
    let done_item:Done = Done::new(item_titele);
    Ok(ItemTypes::Done(done_item))
}
else{
    Err("not the right implementation!!")
}
}