fn main() {
    let x = 100;
    match x {
        25 => println!("20"),
        50 => println!("50"),
        100 => println!("100"),
        _ => println!("Other"), // default statment
    }
    let number = 70;
    match number {
        20 | 30 | 40 => println!("20, 30 or 40"),
        50 | 60 | 70 => println!("50, 60 or 70"),
        80 | 90 | 100 => println!("80, 90 or 100"),
        _ => println!("Other"), // default statment
    }
}
