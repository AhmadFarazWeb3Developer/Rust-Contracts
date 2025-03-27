fn main() {
    let tuple = (1, 'a', 3.4);
    let first_element = tuple.0;
    // tuple.0 = 24; error cannot assing , change tuple to mut
    println!("{}", first_element)
}

/*
By default, Rust stores the elements of a tuple with default types; for
example, 10 is stored as i32, 3.4 as f64, and so on.
*/
