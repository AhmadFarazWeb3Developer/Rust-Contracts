/* Types of ENUMS
1. Simple Enum
2. Optional Enum
3. Result Enum


->  data members of enum is called variants and for struct its called fields
*/

//Simple Enum
enum Color {
    Red,
    Green,
    Blue,
}

// Optional Enum

enum OptionalValue<T> { // kuch or kuch bhi nhi
    Some(T),
    None,
}

// Result Enum

enum Result<T, E> { // result itself is the bulidin enum of rust , so thats why using this as my custom we use :: for calling it this enum
    Ok(T),
    Err(E),
}

fn main() {
    // simple enum
    let color: Color = Color::Red;
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }

    // optional enum
    let some_value: OptionalValue<i32> = OptionalValue::Some(42);
    let none_value: OptionalValue<i32> = OptionalValue::None;
    match some_value {
        OptionalValue::Some(value) => println!("Got a value: {} ", value),
        OptionalValue::None => println!("None value"),
    }

    match none_value {
        OptionalValue::Some(value) => println!("Got a value: {} ", value),
        OptionalValue::None => println!("None value"),
    }

    // result enum

    match divide(100, 10) {
        Result::Ok(result) => {
            println!("Result of division {} ", result);
        }
        Result::Err(error) => { eprintln!("Error: {}", error) }
    }
}

fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 { Result::Err(String::from("Division by zero")) } else { Result::Ok(x / y) }
}
