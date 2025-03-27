fn main() {
    let c1 = 'a';
    let c2 = '5';
    let c3 = '\u{263A}';
    println!(" c1 = {} \n c2 = {} \n c3 = {} ", c1, c2, c3);
}

//Characters in Rust are Unicode scalar values and 4 bytes in size. This is
//  different from languages like C++, where characters are represented by 1
//  byte. Since char in Rust takes 4 bytes and is a Unicode value, we can
//  represent much more than simple letters, like emojis and so on.
