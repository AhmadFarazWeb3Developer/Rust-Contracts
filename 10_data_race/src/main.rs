/* A  "Data Race" is like a race condition in that involves multiples
operations accessing the same datra simultaneoulsy. However, in a data race:

1. Two or more pointers are accessing the same data concurrently.
2. At least one of these pointers is attempting to modify the data.
3. There's no mechanism in place to coordinate or synchronize access to the data, leading to
potential inconsisitencies or errors
*/
fn main() {
    let mut s = String::from("Hello");
    // only read case
    let r1 = &s;
    let r2 = &s;

    println!("Race1= {} , Race2= {} ", r1, r2); // Race1= Hello , Race2= Hello

    // read and write

    let r3 = &s; // borrowed for reading
    let r4 = &mut s; // borrowed for mutablity , means may be changed in future

    println!("Race3= {} , Race4= {} ", r3, r4); // ERROR: can either borrow immutable or mutable
}
