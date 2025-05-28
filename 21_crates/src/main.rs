/*
 A crate is a compilation unit in Rust, which is used by the Rust compiler to
 generate either a binary or a library file. When you use rustc
 <file_name>.rs to compile it, this file is treated as a crate. If the file has
 some mod declarations in it, the mod declarations are replaced by the code in
 the module files and then the combined file - a crate - is compiled. By
 default, rustc produces a binary from a crate, which can be controlled using
 the --crate-type flag. If this flag is set to lib, then a library is produced
 instead of a binary. A binary crate is an executable project and has a main()
 method in it. On the other hand, library crates do not have a main method.
 These are a group of components that can be reused in other projects, and
 their program execution begins in the src/lib.rs file. In Rust, the cargo tool
 is used to manage crates. Third-party crates can be downloaded from
 crates.io- the official crates registry site for Rust


eg std library is a crate, which includes a rich biolerplate code
crates are the libraries or depedencies in rust 
*/

fn main() {
    println!("Hello, world!");
}
