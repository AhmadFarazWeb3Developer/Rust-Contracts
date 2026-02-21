#[derive(Debug)]

// #[derive(Debug)] allows the struct to be printed using {:?} or {:#?}
// It automatically implements the Debug trait for us, so we don’t have to write printing code manually
// it must be written for each struct

struct Student {
    name: String,
    roll_no: u8,
    pass: bool,
}

impl Student {
    fn update_student_details(&mut self) {
        self.name = String::from("Ahmad Faraz");
        self.roll_no = 241;
        self.pass = true;
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u16,
    breadth: u16,
}

impl Rectangle {
    fn calculate_area(&self) -> u16 {
        self.width * self.breadth
    }
    fn update_width(&mut self) {
        self.width = 120;
    }
    fn update_breadth(&mut self) {
        self.breadth = 60;
    }
}
fn main() {
    let student1 = Student {
        name: String::from("Ahmad Faraz"),
        roll_no: 25,
        pass: true,
    };

    let student2 = Student {
        name: String::from("Nadar shah"),
        ..student1 // copying student1 data members value
    };

    println!("student 2 : {:#?}", student2);

    let mut student3 = student2; // ownership transfered

    println!("{:#?}", student1); // '#' is used for formatting
    println!("{:#?}", student3);

    student3.update_student_details();
    println!("{:#?}", student3);

    // ----------------------------------------------------------------------
    let mut rectangle: Rectangle = Rectangle {
        width: 100,
        breadth: 20,
    };

    let area = rectangle.calculate_area();
    println!("Area of rectangle is {} sq/m", area);
    println!("Rectangle {:#?} ", rectangle);

    rectangle.update_width();
    rectangle.update_breadth();
    let area = rectangle.calculate_area();
    println!("Area of rectangle is {} sq/m", area);
    println!("Rectangle {:#?} ", rectangle);
}
