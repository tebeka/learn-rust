use std::cmp::PartialOrd;
use std::fmt::Display;

fn main() {
    let v1 = vec![1, 2, 3, 4];
    println!("m int: {}", max(&v1));

    let s1 = vec!["a", "b", "c"];
    println!("m str: {}", max(&s1));

    let p = Pair::new(1, 2);
    println!("p = {:?}", p);

    let p2 = Pair::new("1", "2");
    println!("p2 = {:?}", p2);

    /*
    {
        let r;

        {
            let x = 5;
            r = &x;
        }
        println!("r = {}", r);
    }
    */

    let s1 = "hello";
    let s2 = "hi";

    let m = longest(s1, s2);
    println!("m = {}", m);

    let lwn = longest_with_ann("abc", "b", "OMG");
    println!("lwn: {}", lwn);
}

fn longest_with_ann<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("ann: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

fn max<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut m = list[0];
    for &item in list.iter() {
        if item > m {
            m = item;
        }
    }

    m
}
