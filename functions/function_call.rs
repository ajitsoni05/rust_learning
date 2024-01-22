fn main(){

   let a=do_stuff(2.1,4.2);
   println!("{}",a);

   let b=do_stuff_2(2.1,4.2);
   println!("{}",b);
}

fn do_stuff(qty:f64, oz :f64) -> f64{

    // returning product of qty and oz
    return qty*oz;
}

fn do_stuff_2(qty:f64, oz :f64) -> f64{
    // returning product of qty and oz using tailwing expression
    qty*oz
}