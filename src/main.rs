// Storing Closures Using Generic Parameters and the Fn Traits
fn main() {
    let subtract_five = |num| num - 5;
    let container = Container::new(subtract_five);
    let result = container.get_closure(25);

    println!("result is : {}", result)
}

struct Container<T>
where
    T: Fn(u32) -> u32,
{
    closure: T,
}

impl<T> Container<T>
where
    T: Fn(u32) -> u32,
{
    fn new(closure: T) -> Self {
        Container { closure }
    }

    fn get_closure(&self, value: u32) -> u32 {
        (self.closure)(value)
    }
}
