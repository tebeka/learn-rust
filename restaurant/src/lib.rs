mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use crate::front_of_house::hosting;
use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    blah();
}

use std::collections::HashMap;

fn blah() -> usize {
    let mut m = HashMap::new();
    m.insert(1, 2);
    m.len()
}
