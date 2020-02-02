# StudentVue-rs
[![Build Status](https://travis-ci.com/13laze/StudentVue-rs.svg?token=SpHdVJ8r5mp8isWTpdaF&branch=master)](https://travis-ci.com/13laze/StudentVue-rs)

> An asynchronous Rust API to access various services offered by the StudentVUE App

## Features
- Retrieve grades
- Get student information

## Usage 
```rust
// Using tokio runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // example...
    Ok(())
}
```

## TODO
- [ ] Redo XML Parsing
- [ ] Basic client functionality 
- [ ] Use Less Allocations
- [ ] Documentation
- [ ] Full Unit Testing
- [ ] Code redo

## License
MIT
