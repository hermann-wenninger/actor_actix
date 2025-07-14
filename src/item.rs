pub struct NewItem {
    pub title: String,
    pub status: String,
    pub user_id: i32,
}

impl NewItem {
    pub fn new(title: String, user_id: i32) -> NewItem {
        return NewItem{title, status: String::from("pending"), user_id}
    }
}
pub struct News{
    pub title:String,
    pub description:String,
    pub date: u32,
}