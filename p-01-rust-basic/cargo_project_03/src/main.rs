fn main() {
    // Tutorial - https://www.youtube.com/watch?v=t047Hseyj_k&list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ&index=4
    // Docs - https://doc.rust-lang.org/book/ch03-02-data-types.html
    println!("Hello, world!");

    // Scaler types
    let floating_point: f32 = 10.95;

    let true_or_false: bool = true;

    let letter: char = 'A';


    // Compound types (array or tuple)
    let tup: (i32, bool, char) = (1, true, 's'); // Immutable

    println!("Index 0:{}", tup.0);


    let mut arr: [u32; 5] = [1, 5, 10, 15, 12];
    arr[2] = 11;
    println!("Array index 2: {}", arr[2]);


}
