use std::io;

use rand::Rng;

// we can view the documentation for all the dependencies we're using with the command:
//      cargo doc --open

fn main() {
    println!("Guessing the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}")
}
