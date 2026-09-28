use crate::calculator::{Operation, SquareRoot, Circle};
use std::io::{self};

mod calculator; 

fn main() {
    
    let mut input: String = String::new();

    println!("Choose operation:");
    println!("-------------------");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");
    println!("5. Power");
    println!("6. Square Root");
    println!("7. PI");

    io::stdin().read_line(&mut input).unwrap();
    let choice: u8 = input.trim().parse().unwrap();

    if choice == 6 {
        println!("Enter Number:");

        let sqrt = calculator::get_number(&mut input);

        let calculation = SquareRoot {
            value: sqrt,
        };

        let result_sqrt = calculator::square_root(calculation);
        println!("Result: {}", result_sqrt);
        return;
    }

    if choice == 7 {
        let pi: f64 = 3.14159;

        input.clear();

        println!("How do you want to Calculate?");
        println!("1. Circumference:");
        println!("2. Area:");
        println!("3. Diameter:");

        let pi_choice: u8 = input.trim().parse().unwrap();

        input.clear();

        println!("Enter radius: "); 

        let radius = calculator::get_number(&mut input);

        let circle = Circle {
            radius,
        };

        match pi_choice {
            1 => {
                let result = 2.0 * pi * circle.radius;
            println!("Circumference: {}", result);
            }
            2 => {
                let result = (circle.radius * circle.radius) * pi;
            println!("Area: {}", result);
            }
            3 => {
                let result = circle.radius * 2.0;
                println!("Diameter: {}", result);
            }
            _ => {
                println!("Invalid Choice.");
            }
        }

        return;
    }

    input.clear();

    println!("Enter first number:");
    let a: f64 = calculator::get_number(&mut input);

    input.clear();

    println!("Enter second number:");
    let b: f64 = calculator::get_number(&mut input);

    let operation: Operation = match choice {
        1 => Operation::Add,
        2 => Operation::Subtract,
        3 => Operation::Multiply,
        4 => Operation::Divide,
        5 => Operation::Power,
        _ => Operation::Invalid,
    };

    let result = calculator::calculate(a, b, operation);
    println!("Result: {}", result)
}
