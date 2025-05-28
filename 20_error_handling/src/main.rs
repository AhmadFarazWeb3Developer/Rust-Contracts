/*
Error Categories:

-> Recoverable Error – A recoverable error, such as if you try to open a file and that 
operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.

-> Unrecoverable Error – Unrecoverable errors are always symptoms of bugs, such as trying to access a location 
beyond the end of an array, and so we want to immediately stop the program.

Note – Most languages don’t distinguish between these two kinds of errors and handle both in the same way,
 using mechanisms such as exceptions. **Rust doesn’t have exceptions.** Instead, it has the type `Result<T, E>`
  for recoverable errors and the `panic!` macro that **stops execution** when the program encounters an unrecoverable error.


*/

fn main() {
    panic!("carsh and burn!"); // panic terminate the program
    println!("carsh and burn!"); // unaccessable after panic
}
