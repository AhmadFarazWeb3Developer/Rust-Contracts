fn main() {
    let mut vec1: Vec<i32> = Vec::new();
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);

    println!("{:?}", vec1);

    let mut vec2: Vec<i32> = vec![1, 2, 3, 4, 5, 5, 66, 777, 7];
    println!("{:?}", vec2);
}
