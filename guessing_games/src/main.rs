use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let number_to_guess = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess input:");
        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&number_to_guess) {
            Ordering::Less => println!("+"),
            Ordering::Greater => println!("-"),
            Ordering::Equal => {
                println!("Success");
                break;
            }
        }
    }

}
