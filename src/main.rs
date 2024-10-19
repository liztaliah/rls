use walkdir::WalkDir;
use std::fs;
use clap::Parser;


#[derive(Parser)]
struct Cli {
    #[arg(short, long, help = "list recursively", action = clap::ArgAction::Count)]
    recursive: u8,
    #[arg(default_value = ".")]
    file_path: String,
}

fn main() {
    let cli = Cli::parse();
    match cli.recursive {
        0 => list_dir(&cli.file_path),
        _ => list_recursive(&cli.file_path),
    }
}

fn list_dir(direc: &str) {
    match fs::read_dir(direc) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                Ok(entry) => println!("{:?}", entry.path()),
                Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn list_recursive(direc: &str) {
    for entry in WalkDir::new(direc) {
        match entry {
            Ok(entry) => println!("{}", entry.path().display()),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}