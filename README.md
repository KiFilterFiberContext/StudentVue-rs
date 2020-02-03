# StudentVue-rs
[![Build Status](https://travis-ci.com/13laze/StudentVue-rs.svg?token=SpHdVJ8r5mp8isWTpdaF&branch=master)](https://travis-ci.com/13laze/StudentVue-rs)

> An asynchronous Rust API to access various services offered by the StudentVUE App

## Features
- Retrieve grades
- Get student information

## Usage 
```rust
use studentvue::client::Client;

#[tokio::main]
async fn main() {
    let client = Client::create("https://studentvue.phoenixunion.org", env!("USER"), env!("PWD"));
    let xml_data = client.get_grades(None)
        .await
        .expect("Could not view grades!");

    println!("{}", xml_data);
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
