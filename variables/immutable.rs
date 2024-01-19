fn main(){
    
    // declaring an immutable vaiable
    let a=1;

    // trying it to reassign
    a=5;



    // Error encountered
    /*
        An immutable variable was reassigned.

            Erroneous code example:

            fn main() {
                let x = 3;
                x = 5; // error, reassignment of immutable variable
            }

            By default, variables in Rust are immutable. To fix this error, add the keyword mut after the keyword let when declaring the variable. For
            example:

            fn main() {
                let mut x = 3;
                x = 5;
            }
     */

}