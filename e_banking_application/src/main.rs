use std::io;

fn main() {
    let mut user_balance: f64 = 0.0;
    loop {
        println!("Choose the option");
        println!("1. Deposit Money");
        println!("2. Withdraw Money");
        println!("3. Check Current Balance");
        println!("4. Exit");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("input error");

        let choice: u8 = match input.trim().parse() {
            Ok(choice) => choice,

            Err(_) => {
                println!("only numbers are accepted");
                continue;
            }
        };
        execute(choice, &mut user_balance);
    }
}

fn execute(choice: u8, user_balance: &mut f64) {
    match choice {
        1 => deposit(user_balance),
        2 => withdraw(user_balance),
        3 => fetch_balance(user_balance),
        4 => exit(),
        _ => println!("Wrong Choice, Try Again!"),
    }
}
fn deposit(user_balance: &mut f64) {
    let mut input = String::new();
    println!("Please Enter Your Amount");
    io::stdin().read_line(&mut input).expect("input error");

    let amount: f64 = match input.trim().parse() {
        Ok(choice) => choice,

        Err(_) => {
            println!("only numbers are accepted");
            return;
        }
    };
    *user_balance = *user_balance + amount;
}

fn withdraw(user_balance: &mut f64) {
    let mut input = String::new();
    println!("Please Withdrawal Amount");
    io::stdin().read_line(&mut input).expect("input error");

    let amount: f64 = match input.trim().parse() {
        Ok(choice) => choice,

        Err(_) => {
            println!("only numbers are accepted");
            return;
        }
    };
    if amount <= *user_balance {
        *user_balance = *user_balance - amount;
    } else {
        println!("Enter amount equal to or less than your balance")
    }
}
fn fetch_balance(user_balance: &mut f64) {
    println!("User current balance {}", *user_balance);
}
fn exit() {
    println!("Thanks for using our service");
    std::process::exit(0);
}
