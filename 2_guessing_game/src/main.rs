use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue,
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! The secret number was {secret_number}"),
            Ordering::Greater => println!("Too big! The secret number was {secret_number}"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
