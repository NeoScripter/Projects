use std::process::{Command, Output, Stdio};
use std::io::{self, ErrorKind};
use std::fmt;

#[derive(Debug)]
enum CmdErr {
    NotFound,
    NonZeroExitCode(i32),
    NoPermission,
    Other(io::Error),
}

impl From<io::Error> for CmdErr {
    fn from(error: io::Error) -> CmdErr {
        match error.kind() {
            ErrorKind::PermissionDenied => CmdErr::NoPermission,
            ErrorKind::NotFound => CmdErr::NotFound,
            _ => CmdErr::Other(error),
        }
    }
}

impl fmt::Display for CmdErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CmdErr::NotFound => write!(f, "Error: Command not found."),
            CmdErr::NoPermission => write!(f, "Error: Insufficient permissions to execute the command."),
            CmdErr::NonZeroExitCode(code) => write!(f, "Error: Command exited with non-zero code: {}", code),
            CmdErr::Other(e) => write!(f, "Error: {}", e),
        }
    }
}

fn execute(command: &str) -> Result<Output, CmdErr> {
    let output = Command::new("cmd")
    .args(&["/C", &command])
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .output()?;

    if !output.status.success() {
        return Err(CmdErr::NonZeroExitCode(output.status.code().unwrap_or(-1)))
    }

    Ok(output)
}

fn main() -> Result<(), CmdErr> {
    println!("Enter command");
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let command = s.trim();
    match execute(command) {
        Ok(output) => println!("Command executed successfully! Output: {}", String::from_utf8_lossy(&output.stdout)),
        Err(e) => println!("{}", e),
    }

    Ok(())
}