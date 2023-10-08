// https://doc.rust-lang.org/book/ch03-05-control-flow.html#summary
// - Convert temperatures between Fahrenheit and Celsius.
// - Generate the nth Fibonacci number.
// - Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

fn main() {
    println!("{:.}", f_to_c(212.0));
    println!("{:.}", f_to_c(0.0));
    println!("{:.}", f_to_c(32.0));

    println!("10th fibonacci number: {}", fib(10));
}

// - Convert temperatures between Fahrenheit and Celsius.
fn f_to_c(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}


// - Generate the nth Fibonacci number.
fn fib(n: i8) -> i64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n-1) + fib(n-2)
    }
}



// - Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.



