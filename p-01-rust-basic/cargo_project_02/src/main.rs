fn main() {
    // Tutorial - https://www.youtube.com/watch?v=xYgfW8cIbMA&list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ&index=3
    // Docs - https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
    
    // Can not be changed
    let x = 4; // Dynamic type

    // Explicitly defined type
    let y: u32 = 10; // Static type


    println!("X is: {}", x);
    println!("Y is: {}", y);
    
    
    // Mutable varible
    let mut z = 12;
    z = 20;
    println!("Z is: {}", z);
    
    // Recreating variable is allowed in rust
    let y: u32 = 15;
    println!("Y is: {}", y);

    // Making my own scope
    // Docs - https://doc.rust-lang.org/rust-by-example/scope.html
    {
        let x = 2;
        println!("X is(inside scope): {}", x);
    }
    println!("X is: {}", x);



    // Constant - value and types can not be changed
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("Seconds in a minute: {}", SECONDS_IN_MINUTE);

}
