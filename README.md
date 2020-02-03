# StudentVue-rs
[![Build Status](https://travis-ci.com/13laze/StudentVue-rs.svg?token=SpHdVJ8r5mp8isWTpdaF&branch=master)](https://travis-ci.com/13laze/StudentVue-rs)

> An asynchronous Rust API to access various services offered by the StudentVUE App

## Features
- Retrieve grades
- Get student information

## Usage 
```rust
use studentvue::{
    client::Client,
    enums::*,
    ParamBuilder,
};

#[tokio::main]
async fn main() {
    let client = Client::create("https://studentvue.phoenixunion.org", "4183350", "1Pud95727");
    let gradedata = client.get_grades(Some(2))
        .await.expect("Could not view grades!");

    println!("{}", gradedata);
    Ok(())
}
```

## TODO
- [ ] Basic client functionality 
- [ ] XML Parsing
- [ ] Reduce Allocations
- [ ] Documentation
- [ ] Full Unit Testing

## License
MIT
