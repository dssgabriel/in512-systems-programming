//! # Guess my number
//! This project's goal is to find the right number between 1 and 100  
//! ## Run commands  
//! Project can be run with `cargo run`  
//! Unit test `cargo test`

use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// Reads user input from `stdin` and parses it to `u8`.
/// Returns the result of the function with an `Option<u8>`.
fn get_int_as_number() -> Option<u8> {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Cannot read input");

    match guess.trim().parse::<u8>() {
        Ok(val) => Some(val),
        Err(_) => {
            eprintln!("error: value cannot be parsed into a `u8`");
            None
        }
    }
}

/// Compares the specified arguments using `std::cmp::Ordering`.
/// Returns the ordering of the first one compared to the second one.
///
/// # Example
/// ```
/// let ordering = get_ordering(3, 5);
/// assert_eq!(ordering, Ordering::Less);
/// ```
fn get_ordering(secret_number: u8, input: u8) -> Ordering {
    match input.cmp(&secret_number) {
        Ordering::Equal => Ordering::Equal,
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
    }
}

/// Prints an hint to the `stdout` given the specified ordering.
fn display_result(result: Ordering) {
    match result {
        Ordering::Less => println!("Your guess is too small!"),
        Ordering::Greater => println!("Your guess is too big!"),
        Ordering::Equal => println!("Congrats, you guessed right!"),
    };
}

/// Returns `true` if the user is found the secret number, false otherwise.
fn has_found(result: Ordering) -> bool {
    match result {
        Ordering::Equal => true,
        _ => false,
    }
}

fn main() {
    println!("Guess my number!");

    let secret_number: u8 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Input your guess: ");

        let guess = match get_int_as_number() {
            Some(val) => {
                if val > 100 {
                    eprintln!("Error: guess is greater than 100");
                    continue;
                } else {
                    val
                }
            }
            None => continue,
        };

        display_result(get_ordering(secret_number, guess));
        if let true = has_found(get_ordering(secret_number, guess)) {
            break;
        } else {
            continue;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twelve_equals_twelve() {
        assert_eq!(get_ordering(12, 12), Ordering::Equal);
    }

    #[test]
    fn twelve_greater_than_five() {
        assert_eq!(get_ordering(5, 12), Ordering::Greater);
    }

    #[test]
    fn twelve_lesser_than_twenty() {
        assert_eq!(get_ordering(20, 12), Ordering::Less);
    }
}
