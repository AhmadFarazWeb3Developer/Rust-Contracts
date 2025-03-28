fn main() {
    let marks = 56;
    if_else(marks);
    matching(4);
    if_let(12);
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

// understand in future
fn if_let(code: u8) {
    if let pattern = code {
        println!("Code {} ", code);
    } else {
        println!("Status Failed {} ", code);
    }
}
