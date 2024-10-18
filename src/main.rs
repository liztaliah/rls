use std::{env, fs};
use clap::{Parser, Subcommand};

fn main() {
    let args = Args::parse();

    if args.len() <= 1 {
        list_dir(".");
    }
    else {
        list_dir(args[1].as_str());
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

#[derive(Parser)]
struct Args {
    #[clap(long, default_value = ".")]
    path: String,
}