use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Generating secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);

    // Processing the guess
    loop {
        let mut guess = String::new();

        io::stdin()
            // if the user types 5 and presses enter, guess looks like this: 5\n
            .read_line(&mut guess)
            .expect("Failed to read line");
        // note: we are "shadowing" the original mutable variable which was a string
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // compare the guess to the generated number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
