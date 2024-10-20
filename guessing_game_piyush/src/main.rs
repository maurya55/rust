use std::io;

use rand::Rng;

use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("secret number {secret_number}");

    loop {
        println!("Please inout your guess number");

        let mut guess: String = String::new();

        let _ = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        // let guess: u32 = guess.trim().parse().expect("failed to parse");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) =>num,
            Err(_) => {
                println!("Please enter valid number");
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You won");
                break;
            }
            Ordering::Greater => println!("To Big"),
            Ordering::Less => println!("To small"),
        }
    }
}
