use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the numeber!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {

        println!("Please enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Gussed: {}",guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
        
}

