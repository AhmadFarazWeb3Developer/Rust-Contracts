#[derive(Debug)]
struct Student {
    name: String,
    roll_no: u8,
    pass: bool,
}

fn main() {
    let student1: Student = Student {
        name: String::from("Ahmad Faraz"),
        roll_no: 25,
        pass: true,
    };

    let student2 = Student {
        name: String::from("Nadar shsh"),
        ..student1 // copying student two data members value
    };
    println!("{:#?}", student2);

    let student3 = student2; // ownership transfered

    println!("{:#?}", student1); // '#' is used for formatting
    println!("{:#?}", student3);
}
