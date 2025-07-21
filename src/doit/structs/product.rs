pub struct Product{
    name:String,
    number:u32,
}
impl Product{
    pub fn new(self,name:String,number:u32)->Product{
        println!("{self.name}-self.number");
        self
    }
    pub selled(self)->(){
        println!("{self.name}{self.number} - is selled");
    }
    
}