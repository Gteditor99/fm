use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct DirectoryMetadata {
    name: String,
    description: String,
}

fn create_directory_with_metadata(name: &str, description: &str) -> Result<(), io::Error> {
    fs::create_dir(name)?;

    let metadata_file = format!("{}/.RootFM", name);
    let mut file = File::create(&metadata_file)?;

    let metadata = DirectoryMetadata {
        name: name.to_owned(),
        description: description.to_owned(),
    };

    let serialized_metadata = serde_json::to_string(&metadata)?;

    file.write_all(serialized_metadata.as_bytes())?;

    Ok(())
}

fn view_directory_metadata(name: &str) -> Result<(), io::Error> {
    let metadata_file = format!("{}/.RootFM", name);
    let file_contents = fs::read_to_string(&metadata_file)?;
    let metadata: DirectoryMetadata = serde_json::from_str(&file_contents)?;

    println!("Metadata for directory '{}':", name);
    println!("Name: {}", metadata.name);
    println!("Description: {}", metadata.description);

    Ok(())
}

// fn remove_directory_with_metadata(name: &str) -> Result<(), io::Error> {
//     let metadata_file = format!("{}/.RootFM", name);
//     fs::remove_dir_all(name)?;
//     fs::remove_file(metadata_file)?;

//     Ok(())
// }

//err: Error removing directory: The system cannot find the path specified. (os error 3)

fn remove_directory_with_metadata(name: &str) -> Result<(), io::Error> {
    let metadata_file = format!("{}/.RootFM", name);
    fs::remove_file(metadata_file)?;
    fs::remove_dir_all(name)?;

    Ok(())
}


fn list_directories_with_metadata() -> Result<(), io::Error> {
    let current_dir = env::current_dir()?;
    let entries = fs::read_dir(&current_dir)?;

    let mut found_directory = false;

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                let metadata_file = path.join(".RootFM");
                if metadata_file.is_file() {
                    if let Some(dir_name) = path.file_name() {
                        let file_contents = fs::read_to_string(metadata_file)?;
                        let metadata: DirectoryMetadata = serde_json::from_str(&file_contents)?;

                        println!("{}", dir_name.to_string_lossy());
                        println!("\tName: {}", metadata.name);
                        println!("\tDescription: {}", metadata.description);
                        println!();

                        found_directory = true;
                    }
                }
            }
        }
    }

    if !found_directory {
        println!("No directories with metadata found.");
    }

    Ok(())
}

fn print_help() {
    println!("Usage: fm [command] [directory_name]");
    println!("Commands:");
    println!("  new | n | create      Create a new directory with metadata");
    println!("  view                  View metadata of an existing directory");
    println!("  remove | rm           Remove a directory and its metadata");
    println!("  list                  List all directories with metadata");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "new" | "n" | "create"=> {
            if args.len() < 4 {
                println!("Error: Directory name or description is missing.");
                return;
            }

            let directory_name = &args[2];
            let directory_description = &args[3];

            match create_directory_with_metadata(directory_name, directory_description) {
                Ok(_) => {
                    println!("Directory '{}' created successfully.", directory_name);
                }
                Err(e) => eprintln!("Error creating directory: {}", e),
            }
        }
        "view" => {
            if args.len() < 3 {
                println!("Error: Directory name is missing.");
                return;
            }

            let directory_name = &args[2];

            view_directory_metadata(directory_name)
                .unwrap_or_else(|e| eprintln!("Error viewing directory metadata: {}", e));
        }
        "remove" | "rm" => {
            if args.len() < 3 {
                println!("Error: Directory name is missing.");
                return;
            }

            let directory_name = &args[2];

            match remove_directory_with_metadata(directory_name) {
                Ok(_) => {
                    println!("Directory '{}' removed successfully.", directory_name);
                }
                Err(e) => eprintln!("Error removing directory: {}", e),
            }
        }
        "list" => {
            list_directories_with_metadata()
                .unwrap_or_else(|e| eprintln!("Error listing directories with metadata: {}", e));
        }
        _ => {
            println!("Error: Invalid command '{}'", command);
            print_help();
        }
    }
}
