fn main(){

   let a=do_stuff(2.1,4.2);
   println!("{}",a);
}

fn do_stuff(qty:f64, oz :f64) -> f64{

    // returning product of qty and oz
    return qty*oz;
}