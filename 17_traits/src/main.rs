/*
Traits are Rust’s take on interfaces or abstract base classes. At first, they look just like
 interfaces in Java or C#. 
*/

struct Student {
    name: String,
}

trait Name {
    fn change_name(&mut self, new_name: String);
}
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
