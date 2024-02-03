use std::fs::{File, OpenOptions};
use std::io::{Write, Result};
use std::sync::{Arc, Mutex};
use std::thread;

struct Logger {
    file: Mutex<File>,
}

impl Logger {
    fn new(file: File) -> Self {
        Logger {
            file: Mutex::new(file),
        }
    }

    fn log(&self, message: &str) -> Result<()> {
        let mut file = self.file.lock().unwrap();
        writeln!(file, "{}", message)?;
        file.flush()?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("bin/logging_system/log.txt")?;

    let logger = Arc::new(Logger::new(file));

    let mut handles = Vec::new();
    for i in 0..5 {
        let logger_clone = Arc::clone(&logger);
        let handle = thread::spawn(move || {
            logger_clone.log(&format!("Log message from thread {}", i)).unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
