use std::{
    sync::mpsc,
    thread,
    fmt,
    error::Error,
};

#[derive(Debug)]
enum ParseError {
    InvalidData(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::InvalidData(ref err) => write!(f, "Invalid data: {}", err),
        }
    }
}

impl Error for ParseError {}

fn main() {
    let values = vec!["4", "7", "29", "t", "beans", "comet"];
    let (tx, rx) = mpsc::channel();

    for v in values {
        let tx_clone = tx.clone();

        thread::spawn(move || {
            match v.parse::<u32>() {
                Ok(num) => {
                    tx_clone.send(Ok(num)).unwrap();
                },
                Err(_) => {
                    let err = ParseError::InvalidData(v.to_string());
                    eprintln!("Thread encountered an error: {}", err);
                    tx_clone.send(Err(err)).unwrap();
                }
            }
        });
    }
    
    drop(tx);

    for received in rx {
        match received {
            Ok(num) => println!("Number {} parsed successfully!", num),
            Err(err) => println!("Main thread received an error: {}", err),
        }
    }
}
