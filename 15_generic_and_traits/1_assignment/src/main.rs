#[derive(Debug)]
struct Container<T> {
    any_value: T,
}

impl<T> Container<T> {
    fn set_value(&mut self, value: T) {
        self.any_value = value;
    }
    fn get_value(&self) -> &T {
        &self.any_value
    }
}

fn main() {
    let mut container: Container<i32> = Container {
        any_value: 12,
    };
    println!("{:#?}", container);
    println!("{:?}", container.get_value());
    container.set_value(100);
    println!("{:?}", container.get_value());
}
