// https://doc.rust-lang.org/book/ch03-05-control-flow.html

fn main() {
    let num = 3;
    if num < 5 {
        println!("true");
    } else {
        println!("false");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    println!("The value is: {}", if 3 < 4 { 5 } else { 6 });

    my_loop();
    for_loop();
}


fn my_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

fn for_loop() {
    let a = [10,20,30,40,50];
    for elem in a {
        println!{"the value is: {elem}"};
    }
    // this prints 3,2,1,
    for num in (1..4).rev() {
        print!("{num},");
    }
}
