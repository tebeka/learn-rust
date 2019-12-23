fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    person();
}

fn person() {
    let p = Person {
        name: String::from("bugs"),
    };
    println!("p = {:?}", p);
}

#[derive(Debug)]
struct Person {
    name: String,
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("dropping {}", self.name)
    }
}
