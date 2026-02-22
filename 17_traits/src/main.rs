/*
A trait is a collection of methods defined for a particular type. Traits are like
interfaces in Java and C# and abstract classes in C++. Traits can contain
abstract methods, which are methods without a body, or concrete methods,
which are methods with a body. This is a bit different from interfaces in that
interfaces do not contain concrete methods
*/

// Interface: Only declares method signatures, no method bodies.
// Abstract class: Can declare method signatures and have some methods with a body.

// Trait is a combination:
// Can declare abstract methods (like an interface).
// Can provide default implementations (like abstract class with concrete methods (with an actual body)).
struct Student {
    name: String,
}

// A trait is like a contract or interface.
// Any type implementing this trait must provide a change_name method
// that takes a mutable reference (&mut self) and a String.
trait Name {
    fn change_name(&mut self, new_name: String);
}

// Implementing the Name trait for the Student struct.”
impl Name for Student {
    fn change_name(&mut self, new_name: String) {
        self.name = new_name;
    }
}

fn main() {
    let mut student: Student = Student {
        name: String::from("Ahmad Faraz"),
    };
    student.change_name(String::from("Saqib khan"));

    println!("Student name : {}", student.name);
}
