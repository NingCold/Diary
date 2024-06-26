use std::{cmp::Ordering, io::stdin};

use rand::{thread_rng, Rng};

fn main() {
    println!("Guess the number!");
    let secret_number = thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess:");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
