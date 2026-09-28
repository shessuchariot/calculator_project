pub enum Operation {
    Add = 1,
    Subtract,
    Multiply,
    Divide,
    Power,
    Invalid,
}

pub struct SquareRoot {
    pub value: f64,
}

pub struct Circle {
    pub radius: f64,
}

pub fn square_root(calculation: SquareRoot) -> f64 {
    calculation.value.sqrt()
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
        Operation::Power => a.powf(b),
        Operation::Invalid => 0.0,
    }
}

pub fn get_number(input: &mut String) -> f64 {
    loop {
        input.clear();

        std::io::stdin().read_line(input).unwrap();

        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => println!("That aint a number. No way"),
        };
    }
}