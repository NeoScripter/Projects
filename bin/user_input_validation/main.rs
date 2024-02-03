use std::{
    io,
    num::ParseIntError,
};

#[derive(Debug)]
enum CtmError {
    ParsingError(ParseIntError),
    InvalidInput(String),
    UnderAge(String),
    NotEnoughMoney(String),
}

impl From<ParseIntError> for CtmError {
    fn from(e: ParseIntError) -> Self {
        CtmError::ParsingError(e)
    }
}

fn buy_drinks(input: &str) -> Result<String, CtmError> {
    let age_limit = 18;
    let booze_prize = 40;
    let data: Vec<u8> = input.split_whitespace()
                             .map(|d| d.parse())
                             .collect::<Result<_, _>>()?;
    
    if data.len() < 2 {
        return Err(CtmError::InvalidInput("Invalid input. Please enter your age and money separated by space.".to_string()));
    }
    
    if data[0] < age_limit {
        Err(CtmError::UnderAge(format!("You're too young for that. Minimum age is {}.", age_limit)))
    } else if data[1] < booze_prize {
        Err(CtmError::NotEnoughMoney(format!("You don't have enough money. Minimum required is ${}.", booze_prize)))
    } else {
        Ok("Here's your drink".to_string())
    }
}

fn main() {
    println!("Please enter your age and the amount of money that you have (separated by space):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    match buy_drinks(&input.trim()) {
        Ok(r) => println!("{}", r),
        Err(e) => println!("{:?}", e),
    }
}
