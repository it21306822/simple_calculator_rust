use std::io;
use std::str::FromStr;

fn main() {
    println!("Enter the first number:");
    let num1 = read_number();

    println!("Enter the second number:");
    let num2 = read_number();

    println!("Enter the operation (+, -, *, /):");
    let operation = read_operation();

    let result = calculate(num1, num2, operation);

    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Invalid operation"),
    }
}

fn read_number() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match f64::from_str(input.trim()) {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number:"),
        }
    }
}

fn read_operation() -> char {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let trimmed = input.trim();
        if trimmed.len() == 1 {
            return trimmed.chars().next().unwrap();
        } else {
            println!("Please enter a valid operation symbol (+, -, *, /):");
        }
    }
}

fn calculate(num1: f64, num2: f64, operation: char) -> Option<f64> {
    match operation {
        '+' => Some(num1 + num2),
        '-' => Some(num1 - num2),
        '*' => Some(num1 * num2),
        '/' => {
            if num2 != 0.0 {
                Some(num1 / num2)
            } else {
                println!("Error: Division by zero");
                None
            }
        }
        _ => None,
    }
}
