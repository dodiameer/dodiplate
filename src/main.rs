use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Boilerplate {
    name: String,
    repo: String,
}

impl std::fmt::Display for Boilerplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Boilerplate `{}` at repo `{}`", self.name, self.repo)
    }
}

fn get_boilerplates() -> Result<Vec<Boilerplate>, Box<dyn Error>> {
    let file = fs::read_to_string("boilerplates.json")?;
    let boilerplates: Vec<Boilerplate> = serde_json::from_str(&file)?;
    Ok(boilerplates) 
}

fn find_boilerplate(boilerplates: Vec<Boilerplate>, name: &str) -> Result<Boilerplate, Box<dyn Error>> {
    for boilerplate in boilerplates {
        if boilerplate.name.as_str() == name {
            return Ok(boilerplate);
        }
    }
    panic!("boilerplate not found");
}

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
    let boilerplates = get_boilerplates()?;
    let name = input("Boilerplate name")?;
    let boilerplate = find_boilerplate(boilerplates, &name.to_lowercase())?;
    println!("{}", boilerplate);
    Ok(())
}
