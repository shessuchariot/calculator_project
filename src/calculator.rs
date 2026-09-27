pub enum Operation {
    Add = 1,
    Subtract,
    Multiply,
    Divide,
    Invalid,
}

pub fn calculate(a: f64, b: f64, operation: Operation) -> f64 {
    match operation {
        Operation::Add => a + b,
        Operation::Subtract => a - b,
        Operation::Multiply  => a * b,
        Operation::Divide => {
        if b == 0.0 {
            println!("Cannot divide by zero Cunt!");
            return 0.0;
        }
        a / b
    }
        Operation::Invalid => 0.0,
    }
}