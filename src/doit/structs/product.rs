pub struct Product{
    name:String,
    number:u32,
}
impl Product {
    pub fn new(name: String, number: u32) -> Product {
        println!("{name}-{number}");
        Product { name, number }
    }
    pub fn selled(&self) {
        println!("{}{} - is selled", self.name, self.number);
    }
    
}