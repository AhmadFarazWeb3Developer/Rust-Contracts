/*
-> In Rust, macros are a way to perfrom metaprogramming-
Macros allow you to tell the compiler: ‘Insert this piece of code here automatically’
instead of writing it manually yourself, like
println!()

-> They are a powerful features used to reduce boilerplate, increase flexibility, and
enhance productivity.

-> Unlike functions, macros operate at complile-time, which means they are expanded by
the compiler before the program runs.


-------- Two Types of Macros -------

.1)-> Declerative : defined using macro_rules!. these are the most and easier to use.
e.g- println!(), panic!(), vec![] etc.

Think: “Here’s a template, fill in the blanks.”
You define patterns (placeholders like $x) and what code should appear.


.2)-> Procedural : More advanced and used for custom derive implementations or attribute-like and
function-like macros. e.g- #[derive(Debug)]

Think: “Here’s some input code, let me write new code for you.”

They are functions that generate Rust code at compile-time.
Can look at structs, functions, or attributes and write extra code automatically.

*/

macro_rules! say_hello {
    () => {
        println!("Hello world")
    }; // Compiler literally replaces say_hello!() with println!() before running the program.
       // Works well for repetitive or boilerplate code.
}

// parameterized declerative-macro
//$ is used to define a placeholder values or patterns
// expr is fragment specifier
macro_rules! repeat_message {
    ($msg:expr, $times:expr) => {
        for _ in 0..$times {
            println!("{}", $msg);
        }
    };
}

// ty fragment specifier

macro_rules! create_vector {
    ($type:ty) => {
        fn new_vector() -> Vec<$type> {
            Vec::new()
        }
    };
}

// PROCEDURAL MACRO

/*
Procedural macros allow for more complex code generation. They come in three forms:

. Custom Drive Macros:  Used to derive traits from struct and enums.
. Attribute-like Macros: Applied to items like functions or modules.
. Funtion-like Macros: Look like function calls but process their input.


Think: “Here’s some input code, let me write new code for you.”
They are functions that generate Rust code at compile-time.
Can look at structs, functions, or attributes and write extra code automatically.

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

    let mut my_vec = new_vector();
    create_vector!(i32);
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
