use std::io::{stdin, stdout, Write};
use std::error::Error;

// This is so overenginnered but I could only get it to work this way
fn input(msg: &str) -> Result<String, Box<dyn Error>> {
    let output = stdout();
    let mut handle = output.lock();
    handle.write(msg.as_bytes())?;
    handle.write(b": ")?;
    handle.flush()?;
    let mut usr_input = String::new(); 
    stdin().read_line(&mut usr_input)?;
    Ok(usr_input.clone())
}

fn main() -> Result<(), Box<dyn Error>> {
    let boilerplate_name = input("Boilerplate name")?.to_lowercase();
    println!("{}", boilerplate_name);
    Ok(())
}