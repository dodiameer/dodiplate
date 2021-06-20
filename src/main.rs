mod boilerplate;

use std::error::Error;
use std::io::{stdin, stdout, Write};

// This is so overenginnered but I could only get it to work this way
fn input(msg: &str) -> Result<String, Box<dyn Error>> {
    let output = stdout();
    let mut handle = output.lock();
    handle.write(msg.as_bytes())?;
    handle.write(b": ")?;
    handle.flush()?;
    let mut usr_input = String::new();
    stdin().read_line(&mut usr_input)?;
    Ok(usr_input.trim().to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let boilerplates = boilerplate::get_boilerplates()?;
    println!("---- List of boilerplates ----");
    for boilerplate in &boilerplates {
        println!("{}", boilerplate);
    }
    println!("------------------------------");
    let name = input("Boilerplate name")?;
    let boilerplate = boilerplate::find_boilerplate(boilerplates, &name.to_lowercase())?;
    let output_directory = input("Output directory")?;
    println!("Cloning {}", boilerplate);
    boilerplate::clone_boilerplate(&boilerplate, &output_directory)?;
    println!("Cloned! Your new boilerplate is at `{}`", output_directory);
    Ok(())
}
