use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use chrono::Local;

fn create_directory_with_metadata(name: &str) -> Result<(), io::Error> {
    fs::create_dir(name)?;

    let metadata_file = format!(".{}.RootFM", name);
    let mut file = File::create(&metadata_file)?;
    file.write_all(b"Put your metadata here")?;

    Ok(())
}

fn view_directory_metadata(name: &str) -> Result<(), io::Error> {
    let metadata_file = format!(".{}.RootFM", name);
    let file_contents = fs::read_to_string(&metadata_file)?;
    println!("Metadata for directory '{}': {}", name, file_contents);

    Ok(())
}

fn print_help() {
    println!("Usage: fm [command] [directory_name]");
    println!("Commands:");
    println!("  create    Create a new directory with metadata");
    println!("  view      View metadata of an existing directory");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        if args.len() == 2 {
            println!("Error: Directory name is missing.");
        } else {
            print_help();
        }
        return;
    }

    let command = &args[1];
    let directory_name = &args[2];

    match command.as_str() {
        "create" => {
            match create_directory_with_metadata(directory_name) {
                Ok(_) => {
                    let timestamp = Local::now();
                    println!("Directory '{}' created successfully at {}.", directory_name, timestamp);
                }
                Err(e) => eprintln!("Error creating directory: {}", e),
            }
        }
        "view" => {
            view_directory_metadata(directory_name)
                .unwrap_or_else(|e| eprintln!("Error viewing directory metadata: {}", e));
        }
        _ => {
            println!("Error: Invalid command '{}'", command);
            print_help();
        }
    }
}
