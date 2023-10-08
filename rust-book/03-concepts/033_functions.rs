fn main() {
    another_function(5);
    block();
    println!("The return value: {}", five());
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn block() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

// return values: https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
