use anyhow::Context;
use ecosysyem::MyError;
use std::fs;

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
