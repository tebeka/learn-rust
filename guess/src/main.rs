pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("value {} not in range [1..100]", value);
        }
        Guess { value: value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let g = Guess::new(19);
    println!("g -> {}", g.value());
    Guess::new(102);
}
