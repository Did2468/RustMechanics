use rust_rng::{generate_secret, check_guess}; 
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("--- Welcome to the Guessing Game ---");
    
   
    let secret_number = generate_secret();

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

       
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number! Try again.");
                continue;
            },
        };

        match check_guess(guess, secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct! You nailed it.");
                break;
            }
        }
    }
}
