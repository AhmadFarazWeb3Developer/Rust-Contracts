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

    let mut stack = vec![1, 2, 3]; // the type of values are here vec<i32>
    while let Some(value) = stack.pop() {
        // but here the types are changed by rust for safety to Option<i32>, beacuse it may panic
        // for empty value when go out of index
        print!("{}, ", value);
    }
}
