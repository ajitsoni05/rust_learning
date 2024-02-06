// strings in rust
/*
There are 6 types of strings in rust
but we mostly care about two of them

- first is string slice 
  str, &str (borrow string slice)
  Data in borrowed string slice cannot be 
  modified while in a string it can be 
  modified.
*/
fn main(){

    // creating a borrowed string slice
    let a="😂".to_string();
    println!("{}", a);

    let b= String::from("🐃");

    println!("{}", b);
}