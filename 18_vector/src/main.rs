/*
Vector or Vec<T> is a collection of elements with the same data type. The
elements of a vector are stored in order. Vectors may look similar to arrays in
some ways, but they have some differences. Arrays have a fixed size that
must be known at compile time. Vectors, on the other hand, can dynamically
grow or shrink at runtime. So, you don’t need to know their exact size at
compile time. A vector’s data is stored on the heap, so we need to handle
ownership and borrowing

OR

A contiguous growable array type with heap-allocated contents.

*/

fn main() {
    let mut vec1: Vec<i32> = Vec::new();
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);

    println!("{:?}", vec1);

    let vec2: Vec<i32> = vec![1, 2, 3, 4, 5, 5, 66, 777, 7];
    println!("{:?}", vec2);
}
