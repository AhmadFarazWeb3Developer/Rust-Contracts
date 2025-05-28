/*
CLOSURE - is like a magic box which remember the thing in rust
its make code shorter and good developer.

In Rust. closures an anynomus functions that can caputre variables from
the surronding scope. Closures can be stored in variables, passed as arguments, or returned from functions.
*/

fn main() {
    let add_number = |x: i32| x + 1;
    println!("{}", add_number(22));

    let mut counter = 0;
    let mut increment = || {
        counter = counter + 1;
        println!("counter value : {}", counter);
    };
    increment();
    increment();
    increment();
    increment();
    increment();

    // Addition
    let mut z = 10;
    let sum = |y: i32| { z + y };

    println!("sum {:?}", sum(5));
    println!("sum {:?}", sum(10));

    // Closures with vector for even number filtering

    let values = vec![10, 20, 30, 45, 75, 100];
    let even_vector: Vec<i32> = values
        .into_iter()
        .filter(|num| num % 2 == 0)
        .collect();

    println!("even values {:?}", even_vector); // [10, 20, 30, 100]

    // Ownership in closures , yes closure transfer the ownership

    let a = String::from("hello world");
    let consume_and_return_a = || &a; // without '&' ownership will transfer
    println!("{}", consume_and_return_a());

    println!("{}", a);
}
