use std::fs::{File, metadata};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::process::Command;
use colored::*;

const COMMON_KEY_OFFSET: u64 = 0xE0;

fn main() -> std::io::Result<()> {
    // Clear terminal
    clear_screen();

    // Prompt
    println!("Where is your OTP path?");
    println!("You can drag and drop it in Finder / File Explorer.");

    print!("> ");
    io::stdout().flush().unwrap();

    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Trim whitespace and possible quotes/apostrophes
    let path = input.trim().trim_matches('"').trim_matches('\'');

    // Check if file exists
    if !metadata(path).is_ok() {
        eprintln!("{}", "ERROR! Path does not exist".red());
        std::process::exit(1);
    }

    // Open file and seek to Common Key offset
    let mut file = File::open(path)?;
    file.seek(SeekFrom::Start(COMMON_KEY_OFFSET))?;

    // Read 16 bytes (the key)
    let mut key = [0u8; 16];
    file.read_exact(&mut key)?;

    // Print key in hex
    println!("\nCommon Key:");
    for byte in &key {
        print!("{:02X}", byte);
    }
    println!();

    Ok(())
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .unwrap();
    } else {
        Command::new("clear")
            .status()
            .unwrap();
    }
}
