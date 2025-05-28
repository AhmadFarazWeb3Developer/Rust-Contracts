use std::{ io, task };
fn main() {
    let mut task_list: Vec<String> = Vec::new();
    loop {
        let mut choice = String::new();
        println!("Please enter your choice");
        println!("1. Add task");
        println!("2. View task");
        println!("3. Remove task");
        println!("4. Edit task");
        println!("5. Exit");

        io::stdin().read_line(&mut choice).expect("Invalid input");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => add_task(&mut task_list),
            2 => view_tasks(&task_list),
            3 => remove_task(&mut task_list),
            4 => edit_task(&mut task_list),
            5 => {
                println!("Exiting......");
                break;
            }
            _ => {
                println!("Invalid input: Try Again!");
            }
        }
    }
}

fn add_task(task_list: &mut Vec<String>) {
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

fn view_tasks(task_list: &Vec<String>) {
    if task_list.is_empty() {
        println!("No tasks found");
        return;
    }
    println!("\nTasks List:");
    for (index, task) in task_list.iter().enumerate() {
        println!("{}. {}", index + 1, task);
    }
}

fn remove_task(task_list: &mut Vec<String>) {
    if task_list.is_empty() {
        println!("No tasks are found");
        return;
    }
    view_tasks(task_list);
    println!("Please enter the task number");
    let mut task_number = String::new();
    io::stdin().read_line(&mut task_number).expect("Invalid input");

    match task_number.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= task_list.len() => {
            task_list.remove(num - 1);
            println!("Task is removed");
        }
        _ => println!("Invalid task number"),
    }
}

fn edit_task(task_list: &mut Vec<String>) {
    view_tasks(task_list);
    println!("Please enter the task number");

    let mut task_number = String::new();
    io::stdin().read_line(&mut task_number).expect("Invalid input");

    let task_number: usize = task_number.trim().parse().expect("Invalid input");
    let found_task = task_list.get(task_number - 1);

    if !found_task.is_none() {
        println!("Enter the updated description ");
        let mut new_task = String::new();
        io::stdin().read_line(&mut new_task).expect("Invalid input");
        task_list[task_number - 1] = new_task.trim().to_string();
        println!("Task updated");
    } else {
        print!("Task does not exist")
    }
}
