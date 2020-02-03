type WebResult<T, E=Box<dyn std::error::Error>> = Result<T, E>;

pub mod client;
pub mod enums;
pub mod error;
pub mod parser;
pub mod request;
