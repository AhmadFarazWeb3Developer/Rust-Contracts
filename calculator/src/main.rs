use std::{ f64, io };

fn add(a: u32, b: u32) -> u32 {
    return a + b;
}
fn sub(a: u32, b: u32) -> u32 {
    if a > b {
        return a - b;
    } else {
        return b - a;
    }
}
fn div(a: u32, b: u32) -> Option<f64> {
    // if b == 0 {
    //     return "b is  zero";
    // } Error , cannot return string as float

    if b == 0 {
        return None;
    }
    return Some((a as f64) / (b as f64));
}
fn mul(a: u32, b: u32) -> u32 {
    return a * b;
}
fn square_root(a: u32) -> f64 {
    return (a as f64).sqrt();
}

fn main() {
    // First input
    let mut input_one = String::new();
    println!("Enter your first number : ");
    io::stdin().read_line(&mut input_one).expect("Not valid input");
    let num1: u32 = input_one.trim().parse().expect("Not a valid number");

    // Second input
    let mut input_two = String::new();
    println!("Enter your second number : ");
    io::stdin().read_line(&mut input_two).expect("Not valid input");
    let num2: u32 = input_two.trim().parse().expect("Not a valid number");

    // funcions calling
    let addition = add(num1, num2);
    let subtraction = sub(num1, num2);
    let multiplication = mul(num1, num2);
    let square_root = square_root(num1);
    println!("Addition {}", addition);
    println!("Subtraction {}", subtraction);
    println!("Multiplication {}", multiplication);

    match div(num1, num2) {
        Some(result) => println!("Division {}", result),
        None => println!("The denominator cannot be zero"),
    }
    println!("Square Root {}", square_root);
}
