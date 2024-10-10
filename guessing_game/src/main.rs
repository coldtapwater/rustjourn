use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");

    println!("The secret number is: {}", secret);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess
                            .trim()
                            .parse()
                            .expect("Please type a number!");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("You win!"); break;},
        }
    }

}

