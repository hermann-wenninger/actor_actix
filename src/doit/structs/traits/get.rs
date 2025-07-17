pub trait Get{
    fn get(&self, title:&str){
        println!("publishing ::: {}", title);
    }
}