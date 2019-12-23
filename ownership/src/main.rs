fn main() {
    let s = String::from("hello there, how are you?");
    let w = first_word(&s);
    println!("w = {}", w);

    let s2 = "bananas are yellow";
    let w = first_word(&s2);
    println!("w = {}", w);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
