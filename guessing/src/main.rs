/* From "The Rust Programming Language" */

use std::cmp::Ordering;
use rand::Rng;

const MAX_ATTEMPTS: i32 = 3;

fn main() {

    let mut i: i32 = 1;
    let secret: i32 = rand::thread_rng().gen_range(1..101);
    println!("[Shhhhhh secret number is {}]", secret);
    let mut guess: String = String::new();

    loop {
        println!("Enter a guess - Attempt {}", i);
        guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please insert a valid number");
                continue
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too high!")
        };

        match i.cmp(&MAX_ATTEMPTS) {
            Ordering::Greater | Ordering::Equal => {
                println!("Max attempts number reached ({}), you lose.", MAX_ATTEMPTS);
                break
            },
            _ => {i+=1}
        }
    }

    println!("GAME OVER");
}
