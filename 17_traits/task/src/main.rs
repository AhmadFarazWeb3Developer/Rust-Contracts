// remove white space from a string

use std::io;
fn main() {
    let mut input = String::new();
    println!("Enter a string : ");
    io::stdin().read_line(&mut input).expect("Failed inputs");
    remove_whitespace(&mut input);
    remove_whitespace_usingchaining(&mut input);
    reverse_input(&mut input);
    println!("String {} ", input);
}
fn remove_whitespace(input: &mut String) {
    let words = input.split_whitespace(); // converted to iterator
    println!("{:?}", words);
    let vec: Vec<&str> = words.collect(); // collect each word after each space
    println!(" {:?}", vec);

    let joined_words = vec.join("");
    println!("without chaining {:?}", joined_words);
}
fn remove_whitespace_usingchaining(input: &mut String) {
    let words = input.split_whitespace().collect::<Vec<&str>>().join("");
    println!("using chaining  {:?}", words);
}
fn reverse_input(input: &mut String) {
    let trimed_input = input.trim();
    let reverse_string: String = trimed_input.chars().rev().collect();
    println!("reversed string  {:?}", reverse_string);
}
