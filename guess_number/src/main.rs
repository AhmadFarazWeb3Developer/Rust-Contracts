use rand::Rng;
use std::io;

fn main() {
    let random_number = rand::rng().random_range(1..=5);
    println!("Guess the number between 1 and 5!");

    loop {
        let mut guessed_number = String::new();
        io::stdin().read_line(&mut guessed_number).expect("Failed to read line");

        let guessed_number = match guessed_number.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number");
                continue;
            }
        };

        match guessed_number.cmp(&random_number) {
            std::cmp::Ordering::Less => println!("Your guess is less then random value!"),
            std::cmp::Ordering::Greater => println!("Your guess is greater then random value!"),
            std::cmp::Ordering::Equal => println!("Success! You won!"),
        }
        // if guessed_number < random_number {
        //     println!("Your guess is less then random vaalue!");
        // } else if guessed_number > random_number {
        //     println!("Your guess is greater then random vaalue!");
        // }
        // else {
        //     println!("Success! You won!");
        //     break;
        // }
    }
}
