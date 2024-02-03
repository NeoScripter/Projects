use std::error::Error;
use std::fmt;
use rand::Rng;
use std::result;

#[derive(Debug)]
enum DatabaseError {
    ConnectionError(String),
    TimeoutError(String),
    QuerySyntaxError(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DatabaseError::ConnectionError(ref err) => write!(f, "Connection Error: {}", err),
            DatabaseError::TimeoutError(ref err) => write!(f, "Timeout Error: {}", err),
            DatabaseError::QuerySyntaxError(ref err) => write!(f, "Query Syntax Error: {}", err),
        }
    }
}

impl Error for DatabaseError {}

type Result<T> = result::Result<T, DatabaseError>;

fn execute_query(query: &str) -> Result<String> {
    if query.len() < 10 {
        return Err(DatabaseError::QuerySyntaxError("Query is too short".to_string()));
    }

    let mut rng = rand::thread_rng();
    let error_type: u8 = rng.gen_range(0..3);

    match error_type {
        0 => Err(DatabaseError::ConnectionError("Failed to connect to the database".to_string())),
        1 => Err(DatabaseError::TimeoutError("Query execution timed out".to_string())),
        _ => Ok("Query executed successfully".to_string()),
    }
}

fn main() {
    let queries = vec![
        "SELECT * FROM users",
        "SE", // Simulate a short, invalid query
        "SELECT * FROM posts",
    ];

    for query in queries {
        match execute_query(query) {
            Ok(result) => println!("Success: {}", result),
            Err(e) => match e {
                DatabaseError::ConnectionError(msg) => println!("{}", msg),
                DatabaseError::TimeoutError(msg) => println!("{}", msg),
                DatabaseError::QuerySyntaxError(msg) => println!("{}", msg),
            },
        }
    }
}
