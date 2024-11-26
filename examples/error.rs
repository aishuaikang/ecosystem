use anyhow::Context;
use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Custom error: {0}")]
    Custom(String),
}

fn main() -> anyhow::Result<()> {
    println!("size of MyError is {}", std::mem::size_of::<MyError>()); // 24

    let filename = "foo.txt";
    let _ = fs::File::open(filename).with_context(|| format!("Can not find file: {}", filename))?;

    fail_with_error()?;

    Ok(())
}

fn fail_with_error() -> anyhow::Result<()> {
    Err(MyError::Custom("This is a custom error".to_string()).into())
}
