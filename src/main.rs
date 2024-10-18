use std::fs;

fn main() {
    match fs::read_dir(".") {
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
