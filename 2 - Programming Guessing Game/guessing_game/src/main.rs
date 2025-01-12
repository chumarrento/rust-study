use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        // other way to parse guess variable
        // parse will parse the string to a number anyways, but we choose the type of the variable
        // rust will infer the type of variable, like let guess: i32
        // let guess: i32 = guess.trim().parse().expect("Please type a number!");

        // i like this way, seems like a rust way
        match guess.trim().parse::<i32>() {
            Ok(guess) => match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            },
            Err(_) => println!("Please provide a valid number!"),
        };
    }
}
