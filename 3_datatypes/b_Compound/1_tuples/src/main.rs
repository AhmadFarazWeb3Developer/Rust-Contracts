fn main() {
    let tuple: (u8, &str, f64) = (1, "a", 3.4);
    // let tuple = (1, 'a', 3.4);
    // let first_element = tuple.0;
    // tuple.0 = 24; error cannot assing , change tuple to mut
    // println!("{}", tuple); //throws error
    println!("{:?}", tuple);
}

/*
By default, Rust stores the elements of a tuple with default types; for
example, 10 is stored as i32, 3.4 as f64, and so on.
*/
