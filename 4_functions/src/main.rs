fn main() {
    let _sum = sum(12, 34);
    let (a, b, c) = sum_v2(23, 400);

    println!("{}", _sum);
    println!("a:{}, b:{}, c:{}", a, b, c);

    let str = return_string();

    println!("{}", str)
}

fn sum(x: u8, y: u8) -> u8 {
    x + y // without return keyword but don't add semicolon then
}

fn sum_v2(a: i32, b: i32) -> (i32, i32, i32) {
    return (a, b, a + b);
}

fn return_string() -> String {
    return "Hello World".to_string();
}

/*
| `&str`                     | `String`                    |
| -------------------------- | --------------------------- |
| Borrowed reference         | Owned value                 |
| Immutable                  | Mutable (if declared `mut`) |
| Stored in binary (literal) | Stored on heap              |
| Fixed size                 | Growable                    |


*/
