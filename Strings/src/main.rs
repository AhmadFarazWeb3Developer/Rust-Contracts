fn main() {
    let _str1 = "Fixed size string, can be updated but cannot be pushed data at run time."; // this is fixed size string
    println!("{}", _str1);
    // _my_string.push_str("Updating the string"); Error

    let mut _str2: String = String::from("Dynamic size string."); // stored at heap
    println!("{}", _str2);
    _str2.push_str(" I am updating my str2");
    println!("{}", _str2);
}
