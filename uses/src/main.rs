use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let v = vec![10, 20, 30, 40, 50];
    println!("v = {:?}", v);
    match v.get(20) {
        Some(val) => println!("val = {}", val),
        None => println!("not found"),
    }

    let mut v2 = vec![1, 2, 3];

    for i in &mut v2 {
        *i += 10;
    }
    println!("v2 = {:?}", v2);

    #[derive(Debug)]
    enum Value {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Value::Int(7),
        Value::Float(1.2),
        Value::Text(String::from("hello")),
    ];
    println!("row = {:?}", row);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s = {}", s);

    let g = "שלום";
    for c in g.chars() {
        println!("c = {}", c);
    }
    for b in g.bytes() {
        println!("b = {}", b);
    }

    let mut m = HashMap::new();
    let text = "mary had a little lamb little lamb";
    for word in text.split_whitespace() {
        let count = m.entry(word).or_insert(0);
        *count += 1;
    }
    println!("m = {:?}", m);
    // panic!("oh my!");

    let f = match File::open("Cargo.tomlz") {
        Ok(file) => file,
        //        Err(error) => panic!("Problem opening the file: {:?}", error),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("/tmp/blah") {
                Ok(file) => file,
                Err(error) => panic!("error create: {:?}", error),
            },
            other => panic!("other error: {:?}", other),
        },
    };
    println!("f = {:?}", f);

    let f2 = File::open("Cargo.toml").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("/tmp/bz2z").unwrap_or_else(|error| {
                panic!("can't create: {:?}", error);
            })
        } else {
            panic!("other error: {:?}", error);
        }
    });
    println!("f2 = {:?}", f2);

    let fname = "Cargo.toml";
    match slurp(fname) {
        Ok(content) => println!("<content>\n{}", content),
        Err(e) => println!("error: {}: {}", fname, e),
    }

    // File::open("boya")?;
    Ok(())
}

fn slurp(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}
