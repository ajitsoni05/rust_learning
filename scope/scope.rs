fn main() {

    let x=1;
    {
        let y=1;
        // below print statement works error free
        println!("{} {}", x, y);
        
    }
    // below print statement throws error because y is out of scope
    println!("{} {}", x, y);
    
}