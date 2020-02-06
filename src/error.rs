//! Errors which may occur during xml parsing or when handling a web request
use std::fmt;

/// General StudentVUE error
#[derive(Debug)]
pub enum VueError {
    Format(std::fmt::Error),
    Xml(quick_xml::DeError),
    Request(reqwest::Error)
}

impl From<std::fmt::Error> for VueError {
    fn from(err: std::fmt::Error) -> VueError {
        VueError::Format(err)
    }
}

impl From<quick_xml::DeError> for VueError {
    fn from(err: quick_xml::DeError) -> VueError {
        VueError::Xml(err)
    }
}

impl From<reqwest::Error> for VueError {
    fn from(err: reqwest::Error) -> VueError {
        VueError::Request(err)
    }
}

impl fmt::Display for VueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VueError::Request(e) => write!(f, "An error occured during the server request: {}", e),
            VueError::Format(e) => write!(f, "Formatting error occured: {}", e),
            VueError::Xml(e) => write!(f, "XML parsing error occured{}", e),
        }
    }
}

impl std::error::Error for VueError {}