use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the numberrr");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("Secret num is {secret_num}");

    loop {
        println!("Input your guesssss");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
