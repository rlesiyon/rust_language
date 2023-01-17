use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop { 
        println!("Guess the number!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                println!("You guessed: {num}");
                num
            },
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"), 
            Ordering::Greater => println!("Too Big!!"), 
            Ordering::Equal => {
                println!("You win!!");
                break
            }
        }
    }

}
