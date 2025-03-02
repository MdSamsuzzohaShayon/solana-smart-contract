fn main() {
    // Tutorial - https://youtu.be/MOa7ulhNYc0?list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ
    // Docs - https://doc.rust-lang.org/rust-by-example/flow_control/if_else.html
    let con = (2 as f32) <= 2.1;
    println!("{}", con);

    let food = "fruit";

    if food == "cookie" {
        println!("I Like cookie");
    }else if food == "fruit" {
        println!("I like fruit");
    }else{
        println!("I don't like cookie");
    }
}
