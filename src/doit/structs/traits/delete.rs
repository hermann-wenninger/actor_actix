pub trait Delete{
    fn delete(&self, title: &str){
        println!("{}was deleted!!!", title);
    }
}