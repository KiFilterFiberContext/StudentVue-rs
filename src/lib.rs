#![allow(unused_imports)]

//! # StudentVue
//! StudentVue is a simplistic Rust API to access various services offered by the StudentVUE App
//! in a convenient format.
//!
//! ## Features
//! - Asynchronous
//! - Retrieve grades, school information, schedules, attendance
//! - Support for various API methods

pub use client::ParamBuilder;
pub use client::Client;
pub use enums::Method;
pub use request::WebHandle;

pub mod client;
pub mod enums;
pub mod error;
pub mod request;
pub mod model;