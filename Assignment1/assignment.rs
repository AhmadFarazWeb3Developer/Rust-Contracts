fn concatenate_strings(str1: &str, str2: &str) -> String {
    let mut result = String::new();
    result.push_str(str1);
    result.push_str(str2);
    return result;
}

fn main() {
    let string1 = String::from("Ahmad ");
    let string2 = String::from("Faraz");

    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("Concatenated String: {}", concatenated_string);
}
