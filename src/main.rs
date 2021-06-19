use git2::Repository;
use remove_dir_all::remove_dir_all as rimraff;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Boilerplate {
    name: String,
    repo: String,
}

impl std::fmt::Display for Boilerplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "boilerplate `{}` at repo `{}`", self.name, self.repo)
    }
}

fn get_boilerplates() -> Result<Vec<Boilerplate>, Box<dyn Error>> {
    let boilerplate_bytes = include_bytes!("../boilerplates.json"); // Bundles the file into the binary
    let file = String::from_utf8_lossy(boilerplate_bytes);
    let boilerplates: Vec<Boilerplate> = serde_json::from_str(&file)?;
    Ok(boilerplates)
}

fn find_boilerplate(
    boilerplates: Vec<Boilerplate>,
    name: &str,
) -> Result<Boilerplate, Box<dyn Error>> {
    for boilerplate in boilerplates {
        if boilerplate.name.as_str() == name {
            return Ok(boilerplate);
        }
    }
    panic!("boilerplate not found");
}

fn clone_boilerplate(
    boilerplate: &Boilerplate,
    output_directory: &str,
) -> Result<String, Box<dyn Error>> {
    let clone_path = Path::new(&env::current_dir()?).join(output_directory);
    if clone_path.is_dir() && clone_path.exists() {
        println!(
            "Output directory `{}` already exists. Removing it...",
            output_directory
        );
        rimraff(&clone_path)?;
    }
    Repository::clone(&boilerplate.repo, &clone_path)?;
    rimraff(clone_path.join(".git"))?;
    Ok(clone_path.display().to_string())
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
    let output_directory = input("Output directory")?;
    println!("Cloning {}", boilerplate);
    clone_boilerplate(&boilerplate, &output_directory)?;
    println!("Cloned! Your new boilerplate is at `{}`", output_directory);
    Ok(())
}
