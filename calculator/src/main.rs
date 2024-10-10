use std::io;



const OPTIONS: [&str; 4] = ["+", "-", "*", "/"];

fn operate(first_input: u32, second_input: u32, operation: &str) -> u32 {
    if operation == "+" {
        first_input + second_input
    } else if operation == "-" {
        first_input - second_input
    } else if operation == "*" {
        first_input * second_input
    } else if operation == "/" {
        first_input / second_input
    } else {
        0
    }
}

fn main() {

    let mut first_input: String = String::new();
    println!("Enter a number: ");

    io::stdin()
        .read_line(&mut first_input)
        .expect("Failed to read line");
    let first_input: u32 = first_input
                            .trim()
                            .parse()
                            .expect("Please type a number!");


    let mut second_input: String = String::new();
    println!("Enter a second number: ");

    io::stdin()
        .read_line(&mut second_input)
        .expect("Failed to read line");
    let second_input: u32 = second_input
                            .trim()
                            .parse()
                            .expect("Please type a number!");

    let mut operation = String::new();
    println!("Enter an operation (+, -, *, or /): ");
    if operation.is_empty() {
        loop {
            operation = String::new();
            io::stdin()
                .read_line(&mut operation)
                .expect("Failed to read line");
            if OPTIONS.contains(&operation.trim()) {
                break;
            }
        }
    }
    let operation: &str = operation.trim();
    println!("{} {} {} = {}", first_input, operation, second_input, operate(first_input, second_input, operation));
    
}

