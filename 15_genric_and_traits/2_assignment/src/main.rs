#[derive(Debug)]
struct Pair<T, U> {
    value_one: T,
    value_two: U,
}

impl<T, U> Pair<T, U> {
    fn get_value_one(&self) -> &T {
        &self.value_one
    }
    fn get_value_two(&self) -> &U {
        &self.value_two
    }

    fn swap_value(self) -> Pair<U, T> {
        Pair {
            value_one: self.value_two,
            value_two: self.value_one,
        }
    }
}

fn main() {
    let pair: Pair<i32, f64> = Pair {
        value_one: 1200,
        value_two: 101.001,
    };

    println!("Original pair: {:#?}", pair);
    println!("Value one: {:?}", pair.get_value_one());
    println!("Value two: {:?}", pair.get_value_two());

    let swapped_pair = pair.swap_value();

    println!("Swapped pair: {:#?}", swapped_pair);
    println!("Value one: {:?}", swapped_pair.get_value_one());
    println!("Value two: {:?}", swapped_pair.get_value_two());
}
