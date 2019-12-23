use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut count = 0;
    loop {
        count += 1;
        println!("Please enter your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("failed to read a line");
        println!("You guessed: {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("You win! (in {} guesses)", count);
                break;
            }
        }
    }
}
