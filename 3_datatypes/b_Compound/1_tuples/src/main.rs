fn main() {
    let tuple: (u8, &str, f64) = (1, "a", 3.4);
    // let tuple = (1, 'a', 3.4);
    // let first_element = tuple.0;
    // tuple.0 = 24; error cannot assing , change tuple to mut
    // println!("{}", tuple); //throws error
    println!("{:?}", tuple);

    let result = calculate(12, 34);

    println!("sum {} ", result.0);
    println!("product {} ", result.1);
    println!("product {} ", result.2);

    let (mut a, b) = (10, 20);

    a = 50; // ✅ allowed
            // b = 60; ❌ not allowed
}

/*
By default, Rust stores the elements of a tuple with default types; for
example, 10 is stored as i32, 3.4 as f64, and so on.
*/

/*
🔥 Interview Tip
If they ask:
How does Rust return multiple values?

You say:
Rust doesn’t support multiple return values directly, but we return a tuple and destructure it in the caller.
*/

fn calculate(a: i32, b: i32) -> (i32, i32, i32) {
    let sum = a + b;
    let product = a * b;
    let sub = a - b;
    (sum, product, sub) // do not write semicolon for return , otherwise it will become the statement and will not return anything
}
