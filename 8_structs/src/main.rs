#[derive(Debug)] // its required when what to print an struct
struct Student {
    name: String,
    roll_no: u8,
    pass: bool,
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
