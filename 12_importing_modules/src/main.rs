mod maths {
    pub fn add(a: u8, b: u8) -> u8 {
        a + b
    }
    pub fn sub(a: u8, b: u8) -> u8 {
        a - b
    }
}

use maths::add;
fn main() {
    let result = add(3, 2); // currently not avaiblable in scope
    println!("{}", result);
}
