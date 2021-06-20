use std::error::Error;
use std::io::{stdin, stdout, Write};

// This is so overenginnered but I could only get it to work this way
pub fn input(msg: &str) -> Result<String, Box<dyn Error>> {
    let output = stdout();
    let mut handle = output.lock();
    handle.write(msg.as_bytes())?;
    handle.write(b": ")?;
    handle.flush()?;
    let mut usr_input = String::new();
    stdin().read_line(&mut usr_input)?;
    Ok(usr_input.trim().to_string())
}
