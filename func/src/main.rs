use std::thread;
use std::time::Duration;

fn main() {
    if false {
        let i = 10;
        let r = 7;

        gen_workout(i, r);
    }

    let mut v = vec![1, 2, 3];
    for i in v.iter_mut() {
        *i += 10;
    }
    println!("v = {:?}", v);
    let s: u32 = v.iter().sum();
    println!("sum = {:?}", s);

    let v2: Vec<u32> = vec![1, 2, 3];
    let out: Vec<_> = v2.iter().map(|n| n + 1).collect();
    println!("out: {:?}", out);

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let mine = show_my_size(shoes, 10);
    println!("mine: {:?}", mine);

    for i in Counter::new() {
        println!("i = {}", i)
    }
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("sum = {}", sum);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn show_my_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == size).collect()
}

#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn gen_workout(i: u32, r: u32) {
    let mut long_calc = Catcher::new(|num| {
        println!("slow...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if i < 25 {
        println!("{} pushups", long_calc.value(i));
        println!("next {} situps", long_calc.value(i));
    } else {
        if r == 3 {
            println!("rest");
        } else {
            println!("{} running", long_calc.value(i));
        }
    }
}

struct Catcher<T>
where
    T: Fn(u32) -> u32,
{
    calc: T,
    value: Option<u32>,
}

impl<T> Catcher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calc: T) -> Catcher<T> {
        Catcher { calc, value: None }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calc)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
