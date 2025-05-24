use std::{ io };

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

fn get_number(prompt: &str) -> u32 {
    // loop run until the valid input is not feeded or the number is not returned
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Not a valid input");

        match input.trim().parse::<u32>() {
            Ok(num) => {
                return num;
            }
            Err(_) => println!("Try Again"),
        }
    }
}

fn main() {
    let num1 = get_number("Enter you first number");
    let num2 = get_number("Enter you second number");

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
