use rand::Rng;
use std::io::stdin;
use std::cmp::Ordering;

fn main() {
    
    let stdin = stdin();

    let rnd = rand::thread_rng().gen_range(1..101);

    println!("Guess the number!");

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        stdin.read_line(&mut guess)
                .expect("Failed to read line"); // "guess" is for example 10\n so I trim() it.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}",err);
                continue;
            }
        };
        
        match guess.cmp(&rnd) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("YOU GOT IT!");
                break;
            }
        }

    }
    
    println!("The number was: {}", rnd);

}
