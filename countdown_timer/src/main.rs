use std::io;
use std::thread;
use std::time::Duration;
fn main() {
    let mut input = String::new();
    print!("Enter amount of time");
    io::stdin().read_line(&mut input).expect("Invalid input");

    let timer: u16 = input.trim().parse().expect("Invalid number");
    start_timer(timer);
}
fn start_timer(timer: u16) {
    for i in (1..=timer).rev() {
        println!("Time {}", i);
        thread::sleep(Duration::from_secs(1)); // creating one second delay
    }
}
