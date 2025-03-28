fn main() {
    let mut x = 0;

    // ---- While Loop ----
    println!("While loop:");
    while x <= 25 {
        print!("{}, ", x);
        x += 1;
    }
    println!("");

    // ---- Loop (Infinite until break) ----
    println!("Loop:");
    loop {
        x += 1;
        print!("{}, ", x);
        if x == 50 {
            break;
        }
    }
    println!("");

    // ---- For Loop ----
    println!("For loop:");
    for i in 0..x {
        print!("{}, ", i);
    }
    println!("");

    // ---- While Let Loop ----
    println!("While let loop:");
    let mut numbers = Some(0);
    while let Some(value) = numbers {
        if value > 5 {
            break;
        }
        print!("{}, ", value);
        numbers = if value < 5 { Some(value + 1) } else { None };
    }
    println!("");
}
