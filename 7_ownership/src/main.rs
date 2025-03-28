// ownership, which enables Rust to guarantee memory safety without the need
// for a garbage collector.

fn main() {
    {
        // Scope begins
        let s = String::from("rust"); // s comes into scope
        println!("{}", s);
    } // Scope ends. Value of s is dropped here
    //   println!("{} ", s);  error  not found in this scope

    // --  STACK VS HEAP --
    // If the size of data cannot be known at compile time or the size changes
    //  dynamically, then the data must be stored on the heap

    // data storage on heap
    //  The memory allocator then finds an unused space in
    //  the heap and returns a pointer to that location. The returned pointer is stored
    //  on the stack, and when you want to access the stored data, you need to follow
    //  the pointer.
}
