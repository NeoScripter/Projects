use std::fmt;
use std::num::ParseIntError;
use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
enum FileReaderError {
    IoError(io::Error),
    ParseError(usize, String, ParseIntError),
}

impl fmt::Display for FileReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileReaderError::IoError(ref err) => write!(f, "IO error: {}", err),
            FileReaderError::ParseError(line_num, ref line, ref err) => 
                write!(f, "Failed to parse line {}: '{}'. Error: {}", line_num, line, err),
        }
    }
}

impl From<io::Error> for FileReaderError {
    fn from(err: io::Error) -> FileReaderError {
        FileReaderError::IoError(err)
    }
}

fn read_numbers(path: &str) -> Result<Vec<i32>, FileReaderError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut numbers = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let num: Result<i32, ParseIntError> = line.parse();
        match num {
            Ok(n) => numbers.push(n),
            Err(e) => return Err(FileReaderError::ParseError(index + 1, line, e)),
        }
    }

    Ok(numbers)
}

fn main() {
    let path = "bin/filereader/numbers.txt";
    match read_numbers(path) {
        Ok(numbers) => {
            println!("Numbers: {:?}", numbers);
        },
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}
