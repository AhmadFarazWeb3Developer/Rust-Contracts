use std::ops::Mul;

/*
 --- GENERICS ---  are a way of writing code for different contexts by parameterization
 of data types and traits. It not only helps reduce code duplication but also
 keeps our code clean and concise. It enables us to write code logic without
 worrying about data types. In order to understand the importance of generic
 data types, let me give you a simple example of a function that accepts two
 integers and returns their sum. We can write a function as follows:*/
fn sum_int(x: i32, y: i32) -> i32 {
    x + y
}

/*
 However, what if we want to find the sum of two floating-point numbers
 instead of two integers. The preceding function add_int expects two i32
 parameters and would complain if we provide floating-point values to it as
 arguments. We can define another function sum_float, which accepts two
 f64 values and returns their sum, as shown in the following code snippet:
 */
fn sum_float(x: f64, y: f64) -> f64 {
    x + y
}

/* 
->  Generics helps us achieve exactly that by letting us write a code
logic independent of data types 

-> Generics can be applied to structures, functions, methods, enums, collections,
 and traits. In the following sections of this chapter, we will see examples of
 all of these
*/

// Structs using generic types

#[derive(Debug)]
struct Circle<T> {
    cx: T,
    cy: T,
    r: T,
}

//  Functions using generic types

fn area_rect<T: Mul<Output = T>>(length: T, width: T) -> T {
    // without : Mul<Output = T> throws error
    length * width
}

//  Methods using generic types

impl<T> Circle<T> {
    fn radius(&self) -> &T {
        &self.r
    }
}

fn main() {
    let a = 10;
    let b = 20;
    let c = sum_int(a, b);
    println!("{}", c);

    // generic struct
    let c1: Circle<i32> = Circle {
        cx: 100,
        cy: 200,
        r: 25,
    };
    println!("{:#?}", c1);

    let c1: Circle<f64> = Circle {
        cx: 10.0,
        cy: 20.1,
        r: 25.101,
    };
    println!("{:#?}", c1);
    println!("circle radius {}", c1.radius());

    // generic function

    let result = area_rect(100, 24);
    println!("{}", result);
    let result = area_rect(1.1, 2.4);
    println!("{}", result);
}
