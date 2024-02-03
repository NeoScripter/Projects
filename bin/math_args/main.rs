use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: <operation> <num1> <num2>");
        process::exit(1);
    }

    let operation = &args[1];
    let num1: f64 = args[2].parse().expect("Please provide a valid number for num1");
    let num2: f64 = args[3].parse().expect("Please provide a valid number for num2");

    match operation.as_str() {
        "add" => println!("Result: {}", num1 + num2),
        "sub" => println!("Result: {}", num1 - num2),
        "mul" => println!("Result: {}", num1 * num2),
        "div" => {
            if num2 == 0.0 {
                eprintln!("Error: Division by zero is not allowed.");
                process::exit(1);
            }
            println!("Result: {}", num1 / num2);
        },
        _ => {
            eprintln!("Unsupported operation: '{}'. Supported operations are add, sub, mul, div.", operation);
            process::exit(1);
        }
    }
}
