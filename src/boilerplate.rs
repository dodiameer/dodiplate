use git2::Repository;
use remove_dir_all::remove_dir_all as rimraff;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use anyhow::{Result, Context, Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct Boilerplate {
    name: String,
    repo: String,
}

impl std::fmt::Display for Boilerplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "`{}` at repo `{}`", self.name, self.repo)
    }
}

pub fn get_boilerplates() -> Result<Vec<Boilerplate>> {
    let boilerplate_file = Path::new(&home::home_dir().unwrap()).join(".dodiplate.json");
    if !boilerplate_file.exists() {
        println!(
            "No boilerplate file found. Creating one at `{}`",
            &boilerplate_file.display()
        );
        fs::write(
            &boilerplate_file,
            include_bytes!("../default-boilerplates.json"),
        ).with_context(|| format!("Unable to create file `{}`", &boilerplate_file.display()))?;
    }
    let boilerplate_file = fs::read_to_string(&boilerplate_file).with_context(|| format!("Unable to read file `{}`", &boilerplate_file.display()))?;
    let boilerplates: Vec<Boilerplate> = serde_json::from_str(&boilerplate_file)?;
    Ok(boilerplates)
}

pub fn find_boilerplate(
    boilerplates: Vec<Boilerplate>,
    name: &str,
) -> Result<Boilerplate> {
    for boilerplate in boilerplates {
        if boilerplate.name.as_str() == name {
            return Ok(boilerplate);
        }
    }
    Err(Error::msg(format!("Unable to find boilerplate `{}`", name)))
}

pub fn clone_boilerplate(
    boilerplate: &Boilerplate,
    output_directory: &str,
) -> Result<String> {
    let clone_path = Path::new(&env::current_dir()?).join(output_directory);
    if clone_path.is_dir() && clone_path.exists() {
        println!(
            "Output directory `{}` already exists. Removing it...",
            output_directory
        );
        rimraff(&clone_path).with_context(|| format!("Unable to remove output directory `{}`", &clone_path.display()))?;
    }
    Repository::clone(&boilerplate.repo, &clone_path).with_context(|| "Unable to clone repository")?;
    rimraff(clone_path.join(".git")).with_context(|| format!("Unable to remove `{}`", clone_path.join(".git").display()))?;
    Ok(clone_path.display().to_string())
}
