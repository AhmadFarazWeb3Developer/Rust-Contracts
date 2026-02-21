fn main() {
    /*
    A slice is a reference to a part of a collection like String and vector, and it
    doesn’t have ownership of the data. Slices can be created using a range
    [start_idx..end_idx], where start_idx is the first index in the slice, and
    end_idx is one more than the last index
    */

    let s = String::from("Ahmad Faraz");
    let w1 = &s[0..5];
    let w2 = &s[6..s.len()];
    println!("First name : {}", w1.len());
    println!("Second name : {}", w2.len());

    let s = String::from("Nadar Shah");
    let (w3, w4) = get_slice(&s);

    println!("First name : {}", w3);
    println!("Second name : {}", w4);

    let mut v = vec![10, 20, 30, 40];
    let slice = &mut v[1..3]; // &mut [i32], mutable slice

    slice[0] = 99;
    slice[1] = 88;

    println!("{:?}", v);
}

fn get_slice(s: &String) -> (&str, &str) {
    return (&s[0..5], &s[6..s.len()]);
}
