use rand::Rng;
use std::cmp::Ordering;

/// Compares a guess against a secret number and returns the result.
pub fn check_guess(guess: u32, secret: u32) -> Ordering {
    guess.cmp(&secret)
}

/// Generates a secret number between 1 and 100.
pub fn generate_secret() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}
