/*
Iterators help you iterate over a collection of values, such as arrays,
vectors, and maps . The iter() method on a collection like vector or array
returns an iterator object of the collection, which is aware of:
- current index
- length
- stop point


How it differ from loop ?

We control the loop start and end.
But Iterator do it automatically.


| Loop                 | Iterator        |
| -------------------- | --------------- |
| Manual index         | No index        |
| Easy to make bugs    | Safe            |
| Can go out of bounds | Impossible      |
| Ownership mistakes   | Handled by Rust |

*/

/// A simple counter that produces numbers from 1 to 4.
///
/// The internal `counter` field keeps track of the current value.
struct Counter {
    counter: u32,
}

impl Counter {
    /// Creates a new `Counter` starting at 0.
    ///
    /// We keep this separate from the Iterator implementation
    /// because this is not iterator behavior — it is just
    /// normal constructor logic for the struct.
    ///
    ///    new() creates an instance of Counter.
    fn new() -> Self {
        Counter { counter: 0 }
    }
}

/// Implementing the `Iterator` trait for `Counter`.
///
/// This allows `Counter` to be used in:
/// - `for` loops
/// - `.next()` calls
/// - iterator adapters like `.map()`, `.filter()`, etc.

impl Iterator for Counter {
    /// The type of value this iterator will return.
    /// Each call to `next()` will return a `u32`.
    ///
    type Item = u32; // Associated type

    /// Advances the counter and returns the next value.
    ///
    /// Returns:
    /// - `Some(value)` while counter < 5
    /// - `None` when iteration is finished
    ///
    /// Self tells Rust: “I’m returning an instance of whatever struct I’m implementing.”
    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;
        if self.counter < 5 {
            Some(self.counter)
        } else {
            None
        }
    }
}

fn main() {
    // Integer array
    let mut int_arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Iterating over references (does NOT transfer ownership)
    for element in int_arr.iter() {
        print!("{} ", element);
    }
    println!("\nArray after iter: {:?}", int_arr);

    // String array
    let str_arr = [
        String::from("Ahmad Faraz"),
        String::from("Hello"),
        String::from("World"),
    ];

    // Iterating by reference (safe, ownership not moved)
    for item in &str_arr {
        print!("{} ", item);
    }

    for element in str_arr.iter() {
        print!("{} ", element);
    }

    // Using iterators explicitly
    let mut iterator = int_arr.iter();
    assert_eq!(Some(&1), iterator.next());
    assert_eq!(Some(&2), iterator.next());
    assert_eq!(Some(&3), iterator.next());

    let mut into_iter = int_arr.into_iter(); // Ownership consuming iterator
    assert_eq!(Some(1), into_iter.next());
    assert_eq!(Some(2), into_iter.next());
    assert_eq!(Some(3), into_iter.next());

    // Mutable iteration to modify elements
    let mut int_arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // re-declare after into_iter consumed it
    for item in int_arr.iter_mut() {
        *item += 10;
    }
    println!("Array after iter_mut: {:?}", int_arr);

    // Custom iterator
    let mut counter = Counter::new();
    while let Some(value) = counter.next() {
        println!("Counter value: {}", value);
    }
}
