use std::borrow::Cow;
//Cow clone on write
fn main() {

    println!("Hello, world!");
    let word: &str= "cdefgahc";
    let result = if word.chars().any(|a|a.is_lowercase()){
        Cow::Owned(word.to_uppercase())
     } else{
        Cow::Borrowed(word)
            };
   
    println!("{}",result);
}
