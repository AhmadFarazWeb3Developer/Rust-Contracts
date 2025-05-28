/* 
lifetime is used to prevent the dangling reference,
*/

fn main() {
    println!("result {}", result);

    let a;
    {
        let b = 5;
        a = &b;
    }

    println!("{}", a); // now we are pointing at the a memory which a garbage value
    // because the b value is dropped as the scope completed and you are accesseing the a which
    // gives the error of lifetime , its the dangling reference error

    let x = 10;
    let y = 20;
    let result = largest_number(&x, &y);
    //
}

fn largest_number<'l>(a: &'l i32, b: &'l i32) -> &'l i32 {
    if a > b {
        return a;
    } else {
        return b;
    }
}
