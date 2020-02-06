# StudentVue-rs
[![Build Status](https://travis-ci.com/13laze/StudentVue-rs.svg?token=SpHdVJ8r5mp8isWTpdaF&branch=master)](https://travis-ci.com/13laze/StudentVue-rs)
![crates.io](https://img.shields.io/crates/v/studentvue.svg)

> An asynchronous Rust API to access various services offered by the StudentVUE App

## Features
- Retrieve grades
- Get student information (health, attendance)
- View schedule
- View calendar 

## Usage 
```rust
// example usage
use studentvue::client::Client;

#[tokio::main]
async fn main() {
    let client = Client::create("https://afsd.edupoint.com/", env!("SID"), env!("SPWD"));
    let grades = client.get_grades(None)
        .await
        .expect("Could not retrieve grades!");
    
    grades.gradebook.courses.course.iter().for_each(|x| {
        println!("{}", x.class_name)
    });
}
```
__Docs__: https://crates.io/crates/studentvue

## License
MIT
