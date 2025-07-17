pub trait Edit{
    fn set_to_done(&self, title: &str){
        println!("{} ::: set to done !!!!",title);
    }

    fn set_to_pending(&self, title: &str){
        println!("{} ::: set to pending !!!!",title);
    }
}