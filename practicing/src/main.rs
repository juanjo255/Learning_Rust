pub mod polars;
use crate::polars::using_polars;
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello_world.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Please check the file is correct: {:?}", error),
    };
}

// fn main() {
//     using_polars()

// }
