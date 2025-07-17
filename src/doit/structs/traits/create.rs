pub trait Create{
    fn create(&self, title:&str){
        println!("{} is beeing created", title);
    }
}