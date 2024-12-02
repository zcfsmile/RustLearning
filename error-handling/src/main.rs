// use std::{error::Error, fmt};

// fn main() {
//     println!("Hello, world!");
// }

// #[derive(Debug)]
// struct JsonError {
//     message: String,
//     line: usize,
//     column: usize
// }

// impl fmt::Display for JsonError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}, ({}: {})", self.message, self.line, self.column)
//     }
// }

// impl Error for JsonError {}

use thiserror::Error;

#[derive(Error, Debug)]
#[error("{message:} ({line}: {column})")]
struct JsonError {
    message: String,
    line: usize,
    column: usize
}

use anyhow::{bail, Ok, Result};

fn divide(a: i32, b: i32) -> Result<f64> {
    if b == 0 {
        bail!("Cannot divide by zero");
    }
    Ok(a as f64 / b as f64)
}

fn main() -> Result<()> {

    // JsonError 用例
    let err = JsonError {
        message: "Unexpected token".to_string(),
        line: 10,
        column: 5
    };
    println!("JsonError: {}", err); // JsonError: Unexpected token (10: 5)
    print!("JsonError: {:?}", err); // JsonError: JsonError { message: "Unexpected token", line: 10, column: 5 }% 

    let result = divide(10, 0)?;
    println!("{}", result); // Error: Cannot divide by zero
    Ok(())
}
