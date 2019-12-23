fn main() {
    let a = [10, 20, 30, 40];

    for elem in a.iter() {
        println!("elem = {}", elem);
    }

    for n in (1..4).rev() {
        println!("n = {}", n);
    }
}
