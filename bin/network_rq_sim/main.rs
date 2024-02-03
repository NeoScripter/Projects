use std::{
    fmt,
    error::Error,
};
use rand::Rng;

#[derive(Debug)]
enum NtwError {
    TimeoutError,
    ConnectionDenied,
    InvalidURL,
}

impl fmt::Display for NtwError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            NtwError::TimeoutError => write!(f, "Timeout error"),
            NtwError::ConnectionDenied => write!(f, "Connection denied"),
            NtwError::InvalidURL => write!(f, "Invalid URL"),
        }
    }
}

impl Error for NtwError {}

fn simulate_request(_: &str) -> Result<String, NtwError> {
    let mut rng = rand::thread_rng();
    let error_type: u8 = rng.gen_range(0..4);

    match error_type {
        0 => Err(NtwError::TimeoutError),
        1 => Err(NtwError::InvalidURL),
        2 => Err(NtwError::ConnectionDenied),
        _ => Ok("Request processed successfully".to_string()),
    }
}

fn main() {
    let requests = vec!["first", "second", "third", "fourth"];

    for r in requests {
        match simulate_request(r) {
            Ok(result) => println!("{}", result),
            Err(e) => println!("{}", e),
        }
    }
}
