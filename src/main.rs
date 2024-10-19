// rls
// Liz Hardee -- October 19 2024

// A simple copy of the ls command written in rust
// Has support for recursive listing and clean vs full path listing
use walkdir::WalkDir;
use core::str;
use std::fs;
use clap::Parser;


// derived commandline arguments
#[derive(Parser)]
struct Cli {
    // recursive flag
    #[arg(short, long, help = "list recursively", action = clap::ArgAction::Count)]
    recursive: u8,
    // filepath, defaults to current working directory
    #[arg(default_value = ".")]
    file_path: String,
    // flag to list full path or to files
    #[arg(short, long, help = "list full path", action = clap::ArgAction::Count)]
    full_list: u8,
    // flag to print detailed data
    // further functionality to come
    #[arg(short, long, help = "list detailed file information", 
    action = clap::ArgAction::Count)]
    detailed: u8,
}

fn main() {
    let cli = Cli::parse();
    match cli.recursive {
        0 => full_path_switch(cli.full_list, 
            &cli.file_path, cli.detailed),
        _ => full_path_switch_recursive(cli.full_list, 
            &cli.file_path, cli.detailed),
    }
}

fn full_path_switch(full_path: u8, direc: &str, detailed: u8) {
    match full_path {
        0 => list_dir(direc, detailed),
        _ => list_dir_full(direc),
    }
}

fn full_path_switch_recursive(full_path: u8, direc: &str, detailed: u8) {
    match full_path {
        0 => list_recursive(direc, detailed),
        _ => list_recursive_full(direc),
    }
}

fn list_dir(direc: &str, detailed: u8) {
    match fs::read_dir(direc) {
        Ok(entries) => {
            if detailed >= 1 {
                println!("{:<21} {:<20}", "filename", "filetype");
            }
            for entry in entries {
                let path = entry.unwrap().path().display().to_string();
                if detailed == 0 {
                    println!("{}", cleanup(&path))
                } else  {
                    println!("{:<20}: {}", cleanup(&path), meta_data(path.as_str()));
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
                let path = entry.unwrap().path().display().to_string();
                println!("{}: {}", path, meta_data(path.as_str()));
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn meta_data(path: &str) -> &str {
    let metadata = fs::metadata(path).unwrap();
    if metadata.is_dir() {
        return "directory";
    } else {
        return "file";
    }
}

fn list_recursive(direc: &str, detailed: u8) {
    for entry in WalkDir::new(direc) {
        let path = entry.unwrap().path().display().to_string();
        if detailed == 0 {
            println!("{}", cleanup(&path))
        } else {
            println!("{:<50}: {}", cleanup(&path), meta_data(path.as_str()));
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

fn cleanup(line: &str) -> String {
    let trim_line: Vec<&str> = line.split('\\').collect();
    trim_line[trim_line.len() - 1].to_string()
}
