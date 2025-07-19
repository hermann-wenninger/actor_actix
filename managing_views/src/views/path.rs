pub struct Path{
    pub path: String,
}
impl Path {
    pub fn define(&self, following:&str)->String{
        format!("{}/{}", self.path.to_owned(), following)
    }
}