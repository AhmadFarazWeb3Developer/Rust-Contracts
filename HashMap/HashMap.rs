use std::collections::HashMap;

fn main() {
    // A hash map stores the value against a key

    let mut scores = HashMap::new();

    // Insertion of scores
    scores.insert(String::from("Ahmad Faraz"), 1100);
    scores.insert(String::from("Saad Israr"), 1200);
    scores.insert(String::from("Nadar Shah"), 940);
    scores.insert(String::from("Shahdab Ali"), 230);
    println!("Scores: {:?}", scores); // Scores: {"Ahmad Faraz": 1100, "Saad Israr": 1200, "Nadar Shah": 940, "Shahdab Ali": 230}

    // Accessing/getting value using a key

    let my_score = scores.get("Ahmad Faraz");
    println!("Ahmad Faraz's score is: {:?}", my_score);

    // Removing/deleting a value using a key

    scores.remove("Saad Israr");
    println!("Scores after removing Saad Israr: {:?}", scores); // Scores after removing Saad Israr: {"Ahmad Faraz": 1100, "Nadar Shah": 940, "Shahdab Ali": 230}

    // Iterating through the HashMap

    for (key, value) in &scores {
        println!("{} : {} ", key, value);
    }
}
