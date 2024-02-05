fn main(){
    for num in [1, 2, 3, 4, 5, 6].iter() {
        println!("{}", num)
    }

    let array = [(1, 2), (3, 4)];

    for (x, y) in array.iter() {
        println!("x: {}, y: {}", x, y)
    }

    //ranges in rust.
    /*
    0..10 means 0 is inclusive and 10 is exclusive
     */

    for num in 0..10 {
        println!("{}", num)
    }
    // if we use ..= then end will also be inclusive
    for num in 0..=10 {
        println!("{}", num)
    }
   
}
