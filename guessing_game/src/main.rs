use std::cmp::Ordering;
use std::io;

/// A simple guessing game where the user tries to guess a randomly generated number between 1 and 100.
fn main() {
    println!("GUESSING GAME\n");

    let winning_number = rand::random_range(1..=100);

    loop {
        let mut guessed_number = String::new();

        println!("Guess the number between 1 and 100");
        
        io::stdin()
            .read_line(&mut guessed_number)
            .expect("Failed to read line");

        println!("You guessed: {}", guessed_number.trim());

        let guessed_number: u32 = match guessed_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };

        match guessed_number.cmp(&winning_number) {
            Ordering::Equal => {
                println!("You won!");
                break;
            }
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
        }
    }
}
