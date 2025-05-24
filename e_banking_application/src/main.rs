use std::io;

fn main() {
    loop {
        println!("Choose the option");
        println!("1. Deposit Money");
        println!("2. Withdraw Money");
        println!("3. Check Current Balance");
        println!("4. Exit");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("input error");

        let option: u8 = match input.trim().parse() {
            Ok(option) => option,

            Err(_) => {
                println!("only numbers are accepted");
                continue;
            }
        };

        match option {
            1 => {
                print!("Money Deposited");
                break;
            }
            2 => {
                print!("Withdraw Money");
                break;
            }
            3 => {
                print!("Check Current Balance");
                break;
            }
            4 => {
                break;
            }
            _ => {
                continue;
            }
        }
    }
}
