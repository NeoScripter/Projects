use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io::{self, stdin};
use std::num::ParseIntError;

#[derive(Debug)]
enum MathError {
    Overflow,
    Underflow,
    DivisionByZero,
    ParseError(ParseIntError),
    InvalidInput
}

impl Display for MathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Error: Cannot divide by zero."),
            MathError::Overflow => write!(f, "Error: Operation results in overflow."),
            MathError::Underflow => write!(f, "Error: Operation results in underflow."),
            MathError::ParseError(e) => write!(f, "Error parsing number: {}", e),
            MathError::InvalidInput => write!(f, "Error: Invalid input format. Please enter exactly two numbers."),
        }
    }
}

impl Error for MathError {}

impl From<ParseIntError> for MathError {
    fn from(e: ParseIntError) -> Self {
        MathError::ParseError(e)
    }
}

fn safe_add(num1: u8, num2: u8) -> Result<u8, MathError> {
    num1.checked_add(num2).ok_or(MathError::Overflow)
}

fn safe_subtract(num1: u8, num2: u8) -> Result<u8, MathError> {
    num1.checked_sub(num2).ok_or(MathError::Underflow)
}

fn safe_multiply(num1: u8, num2: u8) -> Result<u8, MathError> {
    num1.checked_mul(num2).ok_or(MathError::Overflow)
}

fn safe_divide(num1: u8, num2: u8) -> Result<u8, MathError> {
    num1.checked_div(num2).ok_or(MathError::DivisionByZero)
}

fn main() -> Result<(), MathError> {
    println!("Enter two numbers (e.g., '15 4'):");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Couldn't read input");

    let numbers: Vec<&str> = input.trim().split_whitespace().collect();
    if numbers.len() != 2 {
        eprintln!("Please enter exactly two numbers.");
        return Err(MathError::InvalidInput);
    }

    let num1: u8 = numbers[0].parse()?;
    let num2: u8 = numbers[1].parse()?;

    let operations: Vec<fn(u8, u8) -> Result<u8, MathError>> = vec![safe_add, safe_subtract, safe_multiply, safe_divide];

    for op in operations {
        match op(num1, num2) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => println!("{}", e),
        }
    }

    Ok(())
}
