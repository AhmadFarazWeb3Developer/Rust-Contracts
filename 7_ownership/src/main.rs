// Ownership in Rust ensures memory safety without a garbage collector.

fn main() {
    // ----- Ownership and Scope -----
    {
        // Scope begins
        let mut s = String::from("rust"); // `s` comes into scope and is allocated on the heap
        s.push_str(" programming"); // Modify the heap-allocated string
        println!("{}", s);
    } // Scope ends. `s` is dropped here automatically.
    // println!("{}", s); // Error: `s` is out of scope

    // ----- Stack vs Heap -----
    // If the size of data cannot be known at compile time or it changes dynamically,
    // then the data must be stored on the heap.

    // The memory allocator finds unused space in the heap and returns a pointer to that location.
    // This pointer is stored on the stack, and to access the stored data, we follow the pointer.

    // ----- Stack Allocation (Copy Trait) -----
    // Primitive data types (like integers) implement the Copy trait,
    // meaning they are copied when assigned to another variable.
    let a = 10;
    let b = a; // `b` gets a copy of `a`
    println!("{}, {}", a, b); // Output: 10, 10

    // ----- Heap Allocation (Move Semantics) -----
    // For non-primitive types like `String` and `Vec<T>`, ownership rules apply.
    let sa = String::from("rust");
    println!("{}", sa);
    println!("Pointer: {:?}, Capacity: {}, Length: {}", sa.as_ptr(), sa.capacity(), sa.len());

    let sb = sa; // Ownership transferred from `sa` to `sb`
    // This operation is called "Move". It ensures only one owner of the underlying string data,
    // preventing memory errors due to double free.

    // println!("{}", sa); throws error of ownership transfered
    println!("{}", sb);
    println!("Pointer: {:?}, Capacity: {}, Length: {}", sb.as_ptr(), sb.capacity(), sb.len());

    // Notice: `sa` is no longer accessible because ownership has moved to `sb`.

    //-----------------------------------------------------------------
    /*
       Passing values to functions has similar behavior as assignment. If we pass a
    data type like String, vector a function as argument, these are moved by
    default and will no longer be usable in the original scope, whereas if we pass
    an integer as argument, we can still use it in the current scope since the value
    is copied. This behavior can be seen in the following code snippet
        */

    let s = String::from("Ahmad Faraz");
    foo_string(s);
    // println!("{}", s); // ERROR: s is moved

    let x = 10;
    foo_int(x);
    println!("{}", x);

    /*

    Just like passing arguments to functions moves data types like String to the
     scope of function, returning a String or similar data type from a function also
     moves (transfers ownership) the string to the scope where it was called, as
     shown in the following code snippet:
     */

    let s = get_string();
    println!("{}", s);

    // --- Clone --- : some time you would need a separete deep copy to work with independent of actucal owner variable

    let s1 = String::from("rust");
    println!("{}", s1);
    println!("Pointer: {:?}, ", s1.as_ptr());

    let s2 = s1.clone(); // Ownership transferred from `sa` to `sb`
    // This operation is called "Move". It ensures only one owner of the underlying string data,
    // preventing memory errors due to double free.

    // println!("{}", sa); throws error of ownership transfered
    println!("{}", s1);
    println!("{}", s2);
    println!("Pointer: {:?}, ", s2.as_ptr()); // s1 pointer is now different from s2

    // ---------------- REFRENCES AND BORROWING -------------------

    /*
     We have seen that when we pass String as an argument to a function, its
     value is moved to the function’s scope and is no longer usable in the current
     context. In order to be able to use it again, the function should return the
     same string as one of the return values, which should be captured in the
     current scope. But this is too much work for a simple task. Luckily, Rust
     supports references, which allows functions to refer to a value without taking
     ownership of it.

     -> By default, references are immutable, that is, they are not allowed to change
        the borrowed value.
    */

    let mut s = String::from("reference");
    foo_string_ref(&mut s); // ‘s’ passed as reference
    println!("{}", s);

    // ------ Checking Correctness ------
    // 1 : correct

    let mut _s = String::from("rust programming");
    let _ref1 = &_s;
    let _ref2 = &_s;
    println!("ref1 : {} ", _ref1);
    println!("ref2 : {} ", _ref2);
    // 2 : correct
    let mut s = String::from("rust");
    let ref1 = &mut s;
    println!("{}", ref1);

    // 3 : incorrect
    let mut _s = String::from("rust");
    let _ref1 = &_s; // dead because its not used , if i use then i will get error
    let ref2 = &mut _s;
    ref2.push_str("programming");
    // println!("{} : ", _ref1); // error
    println!("{} : ", _ref2); //

    // 4 : correct

    let mut s = String::from("rust");
    {
        let ref1 = &mut s;
    } // ref1 goes out of scope
    let ref2 = &mut s;
}

fn foo_string_ref(s: &mut String) {
    println!("Accessing : {} ", s);
    s.push_str(" Updated");
    /*This phenomenon of functions having references as parameters is called
    borrowing because the function just borrows the values and does not own
    them

    -> i have updated with mut, which is not neccessry for not updating the string value
    */
}

fn foo_string(ss: String) {
    // ss comes into scope
    println!("{}", ss);
} // ss goes out of scope and ‘drop’ is called & memory is

fn foo_int(a: i32) {
    println!("{}", a);
}

fn get_string() -> String {
    let ss = String::from("Nadar Shah");
    ss // ss is moved
}
