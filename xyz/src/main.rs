use std::borrow::Cow;
fn main() {

    println!("Hello, world!");
    let word = "cdefgahc";
    let result = if word.chars().any(|a|a.is_lowercase()){
        Cow::Owned(word.to_uppercase())
     } else{
        Cow::Borrowed(word)
            };
   
    println!("{}",result);
}
