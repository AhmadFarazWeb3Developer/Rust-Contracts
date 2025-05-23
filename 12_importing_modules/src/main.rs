/*  first method 
mod maths {
     pub fn add(a: u8, b: u8) -> u8 {
        a + b
    }
     pub fn sub(a: u8, b: u8) -> u8 {
        a - b
     }
 }
            
    */

// use maths::add;
// use maths::sub;

// use maths::*; // to avoid redundency

// second method via importing the maths library

mod math_lib; // file name where the library is
use math_lib::maths::*;

fn main() {
    let result = add(3, 2);
    println!("{}", result);
    let result = sub(3, 2);
    println!("{}", result);
}
