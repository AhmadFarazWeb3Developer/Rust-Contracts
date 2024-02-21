#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

fn main() {
    let person = Person {
        name: String::from("Ahmad Faraz"),
        age: 23,
    };

    println!("Person Name : {:?}", person.name);
    println!("Person Age : {:?}", person.age);
    //assert!(person.age <= 18, "You are under 18"); // You are under 18 , when the assertion is reached the program stops execution

    let details = format!("Hello, {}.You are {} years old ", person.name, person.age); // Hello, Ahmad Faraz.You are 23 years old
    println!("{}", details);
}
/*
The Debug trait in Rust is a trait defined in the standard library (std::fmt::Debug)
 that allows types to format themselves in a programmer-friendly way for debugging
 purposes. This trait is particularly useful when you want to print out values of custom
 types in a readable format during development and debugging.
To make a type printable using the Debug trait, you typically derive or
implement the trait for that type. The derived implementation
 (using #[derive(Debug)]) automatically generates the necessary code to format
  the type for debugging based on its structure. */
