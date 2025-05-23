/*Shadowing:
you can declare a new variable with the same 
name as a previous variable, and the new variable shadows the previous vari
able. Rustaceans say that the first variable is shadowed by the second, which 
means that the second variable’s value is what appears when the variable 
is used. We can shadow a variable by using the same variable’s name and 
repeating the use of the let keyword.
Shadowing is different than marking a variable as mut, because we’ll get 
a compiletime error if we accidentally try to reassign to this variable without 
using the let keyword. By using let, we can perform a few transformations on 
a value but have the variable be immutable after those transformations have 
been completed.
*/

fn main() {
    // example 1
    let x: String = String::from("Hello");
    println!("{}", x);
    let x: i32 = ;
    println!("{}", x);

    // example 2

    let s: String = String::from("Hello");
    // let s: i32 = s.len(); // Error becasue s.len() returns a usize — the default type for sizes and indexing in Rust.
    let s: usize = s.len();
    println!("{}", s)
}
