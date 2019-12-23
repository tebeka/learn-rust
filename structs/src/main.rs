fn main() {
    let r = Rect {
        height: 30,
        width: 40,
    };
    println!("r = {:?}", r);
    println!("area = {}", r.area());

    let r2 = Rect {
        width: 10,
        height: 17,
    };
    println!("ch {}", r.can_hold(&r2));

    let r3 = Rect::square(17);
    println!("r3 = {:?}", r3);
}

#[derive(Debug)]
struct Rect {
    height: u32,
    width: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rect {
        return Rect {
            width: size,
            height: size,
        };
    }
}
