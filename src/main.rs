use std::error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let f = std::fs::File::open("hello.txt")?;

    Ok(())
}
