use std::io;

fn main() {
    // Tutorial - https://www.youtube.com/watch?v=Uwa3P9dBHdA&list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ&index=6
    // Docs - https://doc.rust-lang.org/rust-by-example/types/cast.html
    println!("Hello, world!");


    let x: u8 = 255; // 0 - 255
    let y: u8 = 10;


    let z = x / y;

    println!("{}", z);

    let a = 127_000i64;
    let b = 10_i64;

    let c = a / b;
    println!("{}", c);


    let i = 127_000_i64;
    let j = 10_i32;
    let k  = i / j as i64; // type casting
    println!("{}", k);


    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Expected to read line");
    let int_input: i64 = input.trim().parse().unwrap();
    println!("{}", int_input + 2);

}
