fn main() {
    println!("Entry Point");

    let _sum = sum(12, 34);
    let (a, b, c) = sum_v2(23, 400);
    println!("{}", _sum);
    println!("a:{}, b:{}, c:{}", a, b, c);
    return_string();
}

fn sum(x: u8, y: u8) -> u8 {
    x + y // without return keyword but dont add semicolon then
}

fn sum_v2(a: i32, b: i32) -> (i32, i32, i32) {
    return (a, b, a + b);
}

fn return_string() -> String {
    return "Hello World".to_string();
}
