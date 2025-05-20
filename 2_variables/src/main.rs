// By Default, variables in Rust are
// immutable, that is, their values cannot be modified or mutated once assigned.

fn main() {
    /*  let x=45;
      println!("value is {}",x);
      x=85;  error cannot assign twice to immutable variable `x`

       In order to make a variable mutable, we have to add the mut keyword before
       the variable name, as suggested by the compiler. A
       println!("value is {}",x);
    */

    let mut _mut_var = 100;
    println!("previous value {} ", _mut_var);

    _mut_var = 200;

    println!("updated value {} ", _mut_var);
}

//  {} , serve as placeholder
