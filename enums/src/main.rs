fn main() {
    let v4 = IPAddrKind::V4;
    println!("v4 = {:?}", v4);
    let v6 = IPAddrKind::V6;
    println!("v6 = {:?}", v6);

    /*
    let lb = IPAddr {
        kind: IPAddrKind::V6,
        address: String::from("::1"),
    };
    */
    let lb = IPAddr::V6(String::from("::1"));
    println!("lb: {:?}", lb);

    let m = Message::Move { x: 10, y: 20 };
    println!("m: {:?}", m);

    let c = Coin::Quarter;
    println!("c = {}", c.cents());

    let i = Some(7);
    println!("inc {:?}", inc(i));

    if let Some(value) = i {
        println!("value = {}", value);
    } else {
        println!("none value");
    }
}

fn inc(x: Option<u32>) -> Option<u32> {
    match x {
        None => None,
        Some(val) => Some(val + 1),
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

/*
#[derive(Debug)]
struct IPAddr {
    kind: IPAddrKind,
    address: String,
}
*/

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(i32, i32, i32),
}

#[derive(Debug)]
enum IPAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IPAddrKind {
    V4,
    V6,
}
