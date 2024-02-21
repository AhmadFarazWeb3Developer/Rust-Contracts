fn main() {
    let age = 23;
    println!("My age is {age}"); // This is a regular string interpolation
    // println!("My age is {:?}", age);
    println!("My age is {age:?}");

    assert!(age >= 18, "You are 18+");
}
