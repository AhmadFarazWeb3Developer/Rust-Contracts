/*

In Rust, modules are a way to organize your code into separate namespaces so
it’s easier to manage, reuse, and avoid name conflicts. Think of them like
folders or sections inside your program.


- A module is defined using the mod keyword.
- It groups related functions, structs, enums, constants, traits, and other modules together.
- Modules help structure large Rust programs.

*/

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
