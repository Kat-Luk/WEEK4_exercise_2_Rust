use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|a| a == "write") {
        if args.len() != 4 {
            println!("More arguments needed if not reading.");
            return;
        } else {
            match write_in_file(&args[1], &args[3]) {
                Ok(contents) => {}
                Err(e) => println!("Error writing to file: {}", e),
            }
        }
    } else if args.iter().any(|a| a == "read") {
        if args.len() != 3 {
            println!("Only give three arguments when writing.");
            return;
        } else {
            match read_file(&args[1]) {
                Ok(contents) => println!("The contents of the file:\n\n{}", contents),
                Err(e) => println!("Error reading file: {}", e),
            }
        }
    } else if args.len() == 1 {
        println!("No arguments were given.");
        return;
    } else {
        println!("invalid command");
        return;
    }
}

fn write_in_file(file_path: &str, input: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(input.as_bytes()).expect("Error writing to the file!");
    Ok(())
}

fn read_file(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}