<div align="center">

# FM (File Manager)

FM (File Manager) is a cross-platform command-line tool written in Rust that provides a user-friendly interface for managing directories and their corresponding metadata files.

<div align="left">

## Features

- Create directories and generate `.RootFM` files to store directory-specific data.
- View and edit directory metadata using the intuitive TUI (Text User Interface).
- Create subdirectories within existing directories, each with its own `.SubFM` file.
- Cross-platform compatibility: FM works seamlessly on Windows, macOS, and Linux.

## Installation

To install FM, you'll need to have Rust installed on your system. If you don't have Rust, you can install it from the official website: https://www.rust-lang.org/

Once Rust is installed, open your command prompt or terminal and run the following command:

`cargo install fm-cli`

This will install FM globally on your system.

## Usage

To create a new directory using FM, run the following command:

`fm create Project001`

This will create a directory named `Project001` and generate a corresponding `.RootFM` file to store metadata.

To view and edit the metadata of an existing directory, navigate to the directory in your command prompt or terminal and run:

`fm view`

This will open the TUI interface where you can modify the directory-specific data.

<div align="center">

## Contribution

Contributions are welcome! If you encounter any issues or have suggestions for improvements, please open an issue on the GitHub repository: https://github.com/gteditor99/fm

## License

FM is released under the MIT License. See the [LICENSE](LICENSE) file for details.
