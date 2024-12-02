/*You’ll also see tuples used as a sort of minimal-drama struct type.
We could declare a struct, but that’s pretty heavy notation for something so obvi
ous, so we just used a tuple
*/
fn main() {
    let _emp_info: (&str, u8) = ("Ahmad Faraz", 24); // 0 , 1

    // println!("{} age is {}", emp_info.0, emp_info.1);

    let _emp_name = emp_info.0;
    let _emp_age = emp_info.1;
    // adding new value
    let _emp_data = true;

    // println!("{} age is {} and his data is : {} ", emp_name, emp_age, emp_data);

    // destructuring

    let (_employee_name, _employee_age) = emp_info;
    println!("{} age is {}", _employee_name, _employee_age);
}
