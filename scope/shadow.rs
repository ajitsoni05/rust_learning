fn main() {

    //shadowing a variable
    let a=3;
    {
        let a=2;
        // prints value of a as 2
        println!("{}", a);  
    }
    // prints value of a as 3
    println!("{}",a);
}