fn test_one() {
    // Convention is to use underscore in name
    println!("Test has been called");
}

fn main() {
    test_one();
    add_num(10, 12);

    let num = {
        let x = 11;
        x + 5
    };

    println!("Num is:{}", num);

    let result = sub_num( 20, 10);
    println!("Minus: {}", result);
}

fn add_num(x: i32, y: i32) {
    println!("The sum is:{}", x + y);
}

fn sub_num(x: i32, y: i32) -> i32{
    x - y // we can return like this at the end of the code without using return keyword, 
    // return x - y; // we can also use return keyword
}


