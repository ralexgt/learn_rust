use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Welcome to the guessing game!");
    let secret_number = rand::rng().random_range(1..=100);
    // println!("The scret number is {}", secret_number);
    println!("Guess the number!");

    let mut times_guessed: u32 = 0;

    loop {
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(val) => val,
            Err(_) => {
                println!("Couldn't read the input. Try again.");
                continue;
            }
        };

        // let guess: u32 = guess.trim().parse().expect("Please provide a number.");
        let guess: u32 = match guess.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Please guess a valid number");
                continue;
            }
        };

        times_guessed += 1;
        // println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("Congrats, that's correct");
                println!("You won in {} guesses", times_guessed);
                break;
            }
            Ordering::Greater => println!("Too high"),
        };
        println!("Guess again buddy");
    }
}
