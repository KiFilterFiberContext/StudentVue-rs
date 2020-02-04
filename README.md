# StudentVue-rs
[![Build Status](https://travis-ci.com/13laze/StudentVue-rs.svg?token=SpHdVJ8r5mp8isWTpdaF&branch=master)](https://travis-ci.com/13laze/StudentVue-rs)

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
    let client = Client::create("https://studentvue.phoenixunion.org", "4183350", "1Pud95727");
    let grades = client.get_grades(None)
        .await
        .expect("Could not retrieve grades!");
    
    grades.gradebook.courses.course.iter().for_each(|x|
        println!("{}", x.class_name)
    );
}
```

## TODO
- [ ] Reduce Allocations
- [ ] Documentation
- [ ] Full Unit Testing

## License
MIT
