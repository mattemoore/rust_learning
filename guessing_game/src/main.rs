use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let mut rng = rand::thread_rng();
    let target = rng.gen_range(0, 10);
    loop {
        println!("Input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type in a number.");
                continue;
            }
        };
        println!("Your guess is: {}", guess);

        match guess.cmp(&target) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { println!("You got it!"); break; }
        }
    }

}
