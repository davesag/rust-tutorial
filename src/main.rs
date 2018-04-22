extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn printer(msg: &str) {
    println!("{}", msg);
}

fn main() {
    const GUESS: &str = "Guess the number!";
    const INPUT_GUESS: &str = "Please input your guess.";
    const ERROR_NUMBER: &str = "Please enter a positive number";
    const ERROR_LINE_READ: &str = "Failed to read line";
    const WRONG_TOO_BIG: &str = "Too big";
    const WRONG_TOO_SMALL: &str = "Too small";
    const RIGHT: &str = "Good guess.";

    printer(&GUESS);

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut i: u32 = 0;

    loop {
        i = i + 1;
        printer(INPUT_GUESS);

        // read_line needs a container for its result.
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect(&ERROR_LINE_READ);

        // Rust allows us to shadow the previous value of guess with a new one
        // a match expression is how you handle the parse error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // The _ is a catchall value
                printer(&ERROR_NUMBER);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => printer(WRONG_TOO_SMALL),
            Ordering::Greater => printer(WRONG_TOO_BIG),
            Ordering::Equal => {
                printer(RIGHT);
                println!("You took {} guesses.", i);
                break;
            }
        }
    }
}
