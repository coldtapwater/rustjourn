use std::io;

fn main() {
    println!("Fibonacci Sequence Generator");
    println!("Enter the number of terms to generate:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u32 = input
                    .trim()
                    .parse()
                    .expect("Please enter a valid number");

    println!("Fibonacci sequence up to {} terms:", n);
    for i in 0..n {
        print!("{} ", fibonacci(i));
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}