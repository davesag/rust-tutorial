extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut i: u32 = 0;

    loop {
        i = i + 1;
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Rust allows us to shadow the previous value of guess with a new one
        // a match expression is how you handle the parse error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // The _ is a catchall value
                println!("Please enter a positive number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("You took {} guesses.", i);
                break;
            }
        }
    }
}
