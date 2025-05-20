fn main() {
    let array = [12, 34, 56, 78, 90, 100];
    println!("{}", array[0]); // 12

    // Creating an empty array of 10 indexes

    let _empty_array: [i32; 10];
    // println!("{}", _empty_array[0]); // throws error because cannot access empty indexes , use the
    // below methof fill with 0 values

    let mut _zero_init_array = [0; 10];
    println!("{}", _zero_init_array[1]); // 0

    let length = _zero_init_array.len();
    _zero_init_array[length - 1] = 100; // setting last index value to 100
    println!("{}", _zero_init_array[length - 1]);
}
