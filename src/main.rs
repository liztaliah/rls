use walkdir::WalkDir;
use std::fs;
use clap::Parser;


#[derive(Parser)]
struct Cli {
    #[arg(short, long, help = "list recursively", action = clap::ArgAction::Count)]
    recursive: u8,
    #[arg(default_value = ".")]
    file_path: String,
    #[arg(short, long, help = "list full path", action = clap::ArgAction::Count)]
    full_list: u8,
}

fn main() {
    let cli = Cli::parse();
    match cli.recursive {
        0 => full_path_switch(cli.full_list, &cli.file_path),
        _ => full_path_switch_recursive(cli.full_list, &cli.file_path),
    }
}

fn full_path_switch(full_path: u8, direc: &str) {
    match full_path {
        0 => list_dir(direc),
        _ => list_dir_full(direc),
    }
}

fn full_path_switch_recursive(full_path: u8, direc: &str) {
    match full_path {
        0 => list_recursive(direc),
        _ => list_recursive_full(direc),
    }
}

fn list_dir(direc: &str) {
    match fs::read_dir(direc) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                Ok(entry) => cleanup(entry.path().display().to_string()),
                Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn list_dir_full(direc: &str) {
    match fs::read_dir(direc) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                Ok(entry) => println!("{}", entry.path().display()),
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
            Ok(entry) => cleanup(entry.path().display().to_string()),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn list_recursive_full(direc: &str) {
    for entry in WalkDir::new(direc) {
        match entry {
            Ok(entry) => println!("{}", entry.path().display()),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn cleanup(line: String) {
    let trim_line: Vec<&str> = line.split('\\').collect();
    println!("{}", trim_line[trim_line.len() - 1]);
}