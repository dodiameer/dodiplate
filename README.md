# dodiplate

A tool to clone boilerplate project files from any git repository. Written in Rust.

## Usage

This isn't really meant for general use, I just built it for my own usage. 

If you want to us it, you can clone the repo, update the `boilerplates.json` file to use your own names and repos. 
Then, you can `cargo build` it and put the resulting binary in a place on your `PATH`. Alternatively, change the name 
of the package in `Cargo.toml`, `cargo publish` it, then `cargo install` it again to add it to your `PATH`

## Note

Picking an already existing folder will simply delete that folder and overwrite it - please be careful!
