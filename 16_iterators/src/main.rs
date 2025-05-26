fn main() {
    let mut int_arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for item in int_arr {
        print!("{}", item);
    }

    /*  iterator is used to loop on the reference of elements of array,
    which means iterating on the reference doesnt transfer the ownership */
    for element in int_arr.iter() {
        print!("{}", element);
    }
    println!("{:?}", int_arr);

    let str_arr = [String::from("Ahmad Faraz"), String::from("Hello"), String::from("World")];

    /*
    // this transfer the ownership of the elements
    for item in str_arr {
        println!("{}", item); // ownership is transfering of each element
    }
    
    println!("{:?}", str_arr); // ownership transfered 
    
    */

    // now used the & for reference
    for item in &str_arr {
        print!("{} ", item);
    }
    println!("");
    for element in str_arr.iter() {
        print!("{} ", element);
    }

    // assetion using iterators

    let mut iterator = int_arr.iter();

    assert_eq!(Some(&1), iterator.next());
    assert_eq!(Some(&2), iterator.next());
    assert_eq!(Some(&3), iterator.next());

    let mut iterator = int_arr.into_iter();
    // into_iter() allow you to work without reference
    assert_eq!(Some(1), iterator.next());
    assert_eq!(Some(2), iterator.next());
    assert_eq!(Some(3), iterator.next());

    for item in int_arr.iter_mut() {
        *item += 10;
    }

    println!("{:?}", int_arr); // [11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
}
