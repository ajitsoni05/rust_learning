fn main() {

    let s1=String::from("abc");
    do_stuff(s1);

    println!("{}", s1);
    // Above line will throw an error

}
fn do_stuff(s: String) {
// do nothing
}

