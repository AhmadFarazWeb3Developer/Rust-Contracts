fn main() {
    let marks = 56;
    if_else(marks);
    matching(4);

    if_let(Some(42)); // Box = [42]

    simple_if_else(Some(12));

    print_code(Some(100))
}

fn if_else(marks: u8) {
    if marks >= 90 {
        println!("Grade is A");
    } else if marks >= 80 {
        println!("Grade is A-");
    } else if marks >= 70 {
        println!("Grade is B");
    } else if marks >= 60 {
        println!("Grade is B-");
    } else if marks >= 50 {
        println!("Grade is C");
    } else if marks >= 40 {
        println!("Grade is C-");
    } else if marks >= 30 {
        println!("Grade is D");
    } else {
        println!("Grade is F");
    }
}

//  match expressions are something like the C switch statement, but more flexible. A
//  simple example:
fn matching(code: u8) {
    match code {
        0 => println!("OK"),
        1 => println!("Wires Tangled"),
        2 => println!("User Asleep"),
        _ => println!("Unrecognized Error {}", code),
    }
}

// if let Some(value) = code = pattern matching + value extraction in one line
// Using Some(value) allows you to directly get the value inside the box safely
// Normal if/else or code.is_some() = only checks existence, not extract value

fn if_let(code: Option<u8>) {
    if let Some(value) = code {
        // Pattern Some(value) matches because box is not empty
        // value is inide the Some is the placeholder for the code variable Some(42)
        // value = 42 (take the number out of the box of code = Some(42))
        println!("Code {:?} ", value);
    } else {
        println!("Status Failed {:?} ", code);
    }
}

fn simple_if_else(code: Option<u8>) {
    if code.is_some() {
        println!("Code exists"); // cannot directly get the value
    } else {
        println!("Status Failed");
    }
}

// print_code is your function using .is_some() + .unwrap(). Require extra steps
fn print_code(code: Option<u8>) {
    if code.is_some() {
        let value = code.unwrap();
        println!("Code {}", value);
    } else {
        println!("Status Failed");
    }
}
