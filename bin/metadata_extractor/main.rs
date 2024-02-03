use std::{
    fs::{self, metadata},
    io,
    fmt,
};

#[derive(Debug)]
enum CustomErr {
    NotFound(io::Error),
    AccessDenied(io::Error),
    InvalidPath(io::Error),
}

impl fmt::Display for CustomErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CustomErr::NotFound(_) => write!(f, "File not found"),
            CustomErr::AccessDenied(_) => write!(f, "Access to the file denied"),
            CustomErr::InvalidPath(_) => write!(f, "Invalid path"),
        }
    }
}

impl From<io::Error> for CustomErr {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => CustomErr::NotFound(err),
            io::ErrorKind::PermissionDenied => CustomErr::AccessDenied(err),
            _ => CustomErr::InvalidPath(err),
        }
    }
}

fn retrieve_metadata(path: &str) -> Result<fs::Metadata, CustomErr> {
    let mt = metadata(path)?;
    Ok(mt)
}

fn main() {
    let path = "C:/Users/Ilya/Downloads/Aalborg White Industry 2020 Eng.pdf";
    match retrieve_metadata(path) {
        Ok(metadata) => {
            println!("Successfully retrieved metadata!");
            println!("{:?}", metadata);
        }
        Err(e) => {
            println!("Error retrieving metadata: {}", e);
        }
    }
}
