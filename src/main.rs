use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use cursive::Cursive;
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let dir_path = Path::new(&args[1]);

    let root_fm_path = dir_path.join(".RootFM");
    let mut root_fm_data = String::new();

    if root_fm_path.exists() {
        let root_fm_file = File::open(root_fm_path)?;
        let mut root_fm_reader = BufReader::new(root_fm_file);
        root_fm_reader.read_line(&mut root_fm_data)?;
        println!("{}", root_fm_data.trim());

        // TODO: Prompt user for action (create sub-directory, view data, etc.)
    } else {
        let mut root_fm_file = File::create(root_fm_path)?;
        let mut root_fm_writer = io::stdout();

        println!("Enter data for directory '{}':", dir_path.display());
        io::copy(&mut io::stdin(), &mut root_fm_writer)?;

        root_fm_file.write_all(root_fm_data.as_bytes())?;
    }

    Ok(())
}