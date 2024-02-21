fn newOwner(s: String) {
    println!("newOwner is created: {}", s);
}
fn main() {
        let onwer1 = String::from("Ahmad Faraz is owner");
        let age = Box::from(12);
        println!("{}", onwer1);
        newOwner(onwer1);
        println!("{}", age);

    // Bowrring and Reference

    let mut string1 = String::from("Hello");

    let string2 = &string1; // immutable reference
    println!("String 2 : {}", string1); // String 2 : Hello

    let string3 = &mut string1;

    string3.push_str(" World!");

    println!("String 3 : {} ", string3); //  String 3 : Hello World!
    println!("String 1 : {}", string1); //  String 1 : Hello World!
}
