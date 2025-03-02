use std::io;

fn main() {
    // Tutorial - https://www.youtube.com/watch?v=PQBX-ev5q2k&list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ&index=5
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");

    println!("Hello {}", input);
}
