use std::io;
fn main() {
    let mut task_list: Vec<String> = Vec::new();
    loop {
        let mut choice = String::new();
        println!("Please enter your choice");
        println!("1. Add task");
        println!("2. View task");
        println!("3. Remove task");
        io::stdin().read_line(&mut choice).expect("Invalid input");

        let choice: i32 = choice.trim().parse().expect("Invalid Choice");

        match choice {
            1 => add_tast(&mut task_list),
            2 => view_tast(&task_list),
            3 => remove_tast(&mut task_list),
            4 => {
                println!("Exiting......");
                break;
            }
            _ => {
                println!("Wrong input: Try Again!");
            }
        }
    }
}

fn add_tast(task_list: &mut Vec<String>) {
    let mut description = String::new();
    println!("Enter your description ");
    io::stdin().read_line(&mut description).expect("Invalid input");

    let description = description.trim().to_string();
    if !description.is_empty() {
        task_list.push(description);
    } else {
        println!("Description cannot be empty");
    }
}
fn remove_tast(task_list: &mut Vec<String>) {
    if task_list.is_empty() {
        println!("No tasks are found");
        return;
    }
    println!("Please enter the task number");
    view_tast(task_list);
    let mut task_number = String::new();
    io::stdin().read_line(&mut task_number).expect("Invalid input");

    let task_number: usize = task_number.trim().parse().expect("Invalid input");
    task_list.remove(task_number - 1);
    println!("Task is removed");
}
fn view_tast(task_list: &Vec<String>) {
    if task_list.is_empty() {
        println!("No tasks are found");
        return;
    }

    println!("Task list {:?}", task_list)
}
