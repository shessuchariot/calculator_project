use crate::calculator::Operation;
use std::io;

mod calculator;

fn main() {

    let mut input: String = String::new();

    println!("Choose operation:");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");

    io::stdin().read_line(&mut input).unwrap();
    let choice: u8 = input.trim().parse().unwrap();

    input.clear();

    println!("Enter first number:");
    io::stdin().read_line(&mut input).unwrap();
    let a: f64 = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("That aint no number boy.");
            return;
        }
    };

    input.clear();

    println!("Enter second number:");
    io::stdin().read_line(&mut input).unwrap();
    let b: f64 = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("That aint no number boy.");
            return;
        }
    };

    let operation: Operation = match choice {
        1 => Operation::Add,
        2 => Operation::Subtract,
        3 => Operation::Multiply,
        4 => Operation::Divide,
        _ => Operation::Invalid,
    };

    let result = calculator::calculate(a, b, operation);
    println!("Result: {}", result)
}
