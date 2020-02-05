use std::fmt::{Display, Formatter};

// TODO: elaborate upon error details
#[derive(Debug)]
pub enum VueError {
    WebError(&'static str),
    ParsingError(&'static str),
    Custom(&'static str),
}

impl Display for VueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VueError::WebError(e) => write!(f, "Error occured during request handling: {}", e),
            VueError::ParsingError(e) => write!(f, "Parsing error occured: {}", e),
            VueError::Custom(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for VueError {}