use std::io;
fn main() {
    let mut input = String::new();

    println!("Enter a digit");
    io::stdin().read_line(&mut input).expect("Invalid input");

    let decimal_val: u32 = match input.trim().parse() {
        Ok(decimal_val) => decimal_val,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    binary_converter(decimal_val);
    hex_converter(decimal_val);
}
// mut in argument here creates a copy of decimal_val varibale
fn binary_converter(mut decimal_val: u32) {
    let mut binary = String::new();
    while decimal_val > 0 {
        let reminder = (decimal_val % 2) as u8;

        binary.insert(0, (reminder + 48) as char);
        // print!("{}", reminder); // this print the binary reverse so that why we using string.insert
        decimal_val = decimal_val / 2;
    }
    println!("Binary number {}", binary);
    println!("")
}
fn hex_converter(mut decimal_val: u32) {
    let mut hexadecimal = String::new();
    while decimal_val > 0 {
        let reminder: u8 = (decimal_val % 16) as u8;
        if reminder > 9 {
            hexadecimal.insert(0, (reminder + 55) as char); // hex value of 10 is A. So char value of 65 is A
        } else {
            hexadecimal.insert(0, (reminder + 48) as char);
        }
        decimal_val = decimal_val / 16;
    }

    println!("Hexa number {}", hexadecimal)
}
