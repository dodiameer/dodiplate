mod boilerplate;
mod utils;
use std::error::Error;
use anyhow::{Context, Result};

fn main() -> Result<(), Box<dyn Error>> {
    let boilerplates = boilerplate::get_boilerplates().with_context(|| "Unable to read configuration")?;
    println!("---- List of boilerplates ----");
    for boilerplate in &boilerplates {
        println!("{}", boilerplate);
    }
    println!("------------------------------");
    let name = utils::input("Boilerplate name")?;
    let boilerplate = boilerplate::find_boilerplate(boilerplates, &name.to_lowercase())?;
    let output_directory = utils::input("Output directory")?;
    println!("Cloning {}", boilerplate);
    boilerplate::clone_boilerplate(&boilerplate, &output_directory)?;
    println!("Cloned! Your new boilerplate is at `{}`", output_directory);
    Ok(())
}
