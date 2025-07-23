use serde::Serialize;
use crate::to_do::ItemTypes;
use crate::structs::base::Base;

#[derive(Serialize)]
pub struct TodoItems{
    pub pending_items:Vec<Base>,
    pub done_items:Vec<Base>

}