/*

-> In Rust, macros are a way to perfrom metaprogramming- wrting code that generates other code, like
println!()

-> They are a powerful feature used to reduce boilerplate, increase flexibility, and 
enhance productivity.

-> Unlike functions, macros operate at complile-time, which means they are expanded by 
the compiler before the program runs.


-------- Two Types of Macros -------

.1)-> Declerative : defined usinf macro_rules!. these are the most and easier to use.
e.g- println!(), panic!(), vec![] etc.

.2)-> Procedural : More advanced and used for custom derive implementations or attribute-like and 
function-like macros. e.g- #[derive(Debug)]

*/

macro_rules! say_hello {
    () => {
        println!("Hello world")
    };
}
// parameterized declerative-macro
//$ is used to define a placeholder values or patterns
// expr is fragment specifier
macro_rules! repeat_message {
    ($msg:expr, $times:expr) => {
        for _ in 0..$times{
            println!("{}",$msg);
        }
    };
}

// ty fragment specifier

macro_rules! create_vector {
    ($type:ty) => {
        fn new_vector()->Vec<$type>{
            Vec::new()
        }
    };
}

// PROCEDURAL MACRO

/*
Procedural macros allow for more complex code generation. They come in three forms:

. Custom Drive Macros:  Used to derive trails from struct and enums.
. Attribute-like Macros: Applied to items like functions or modules.
. Funtion-like Macros: Look like function calls but process their input.

*/

use serde::Serialize;
#[derive(Serialize)]
struct User {
    name: String,
    age: u32,
}
fn main() {
    say_hello!();

    repeat_message!("Rust is awesome!", 3);
    create_vector!(i32);
    let mut my_vec = new_vector();
    println!("created a vector of type i32 {:?}", my_vec);

    my_vec.push(23);
    my_vec.push(23);
    my_vec.push(23);
    my_vec.push(23);
    my_vec.push(23);

    println!("pushed values to my vector {:?}", my_vec);

    // using procedual macros

    let user = User {
        name: "Ahmad Faraz".to_string(),
        age: 30,
    };

    println!("{}", serde_json::to_string(&user).unwrap());
}
