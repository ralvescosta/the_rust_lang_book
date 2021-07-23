use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    loop {
        let mut guess = String::new();
        println!("Please input your guess: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Type a int: {}", err);
                continue;
            }
        };
        let secret_number = rand::thread_rng().gen_range(1..101);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}", "You Win!".green());
                break;
            }
            Ordering::Greater => println!("{}", "To Big! Try again...".red()),
            Ordering::Less => println!("{}", "To Small! Try again...".red()),
        }
    }
}
