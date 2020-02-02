type WebResult<T> = Result<T, Box<dyn std::error::Error>>;
use simple_xml_builder::XMLElement;

pub mod client;
pub mod enums;
pub mod error;
pub mod parser;
pub mod request;
