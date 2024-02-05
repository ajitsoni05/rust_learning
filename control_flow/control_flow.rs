fn main(){
    let num=2;
    let mut msg="";

    if num==2{
        msg="two";
    }else if num==3{
        msg="three";
    }else{
        msg="other";
    }
    println!("{}",msg);


    msg = if num==2 {
            "two"
    }else if num==3 {
        "three"
    }else{
        "other"
    };

    println!("{}",msg);


    
}