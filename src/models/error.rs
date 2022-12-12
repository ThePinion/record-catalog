use std::{error, fmt};

use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct AppError(pub String);

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for AppError {}
