type WebResult<T> = Result<T, Box<dyn std::error::Error>>;

pub mod client;
pub mod enums;
pub mod error;
pub mod parser;
pub mod request;
