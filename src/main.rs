use std::fs::{File, metadata};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::process::Command;
use colored::*;

const COMMON_KEY_OFFSET: u64 = 0xE0;

fn main() -> std::io::Result<()> {
    clear_screen();

    println!("Where is your OTP path?");
    println!("You can drag and drop it in Finder / File Explorer.");

    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let path = sanitize_path(&input);

    if !metadata(&path).is_ok() {
        eprintln!("{}", "ERROR! Path does not exist".red());
        std::process::exit(1);
    }

    let mut file = File::open(&path)?;
    file.seek(SeekFrom::Start(COMMON_KEY_OFFSET))?;

    let mut key = [0u8; 16];
    file.read_exact(&mut key)?;

    println!("\nHere is your Common Key:");
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

/// Sanitizes drag-n-drop paths:
/// - Trims whitespace
/// - Removes leading/trailing quotes or apostrophes
/// - Converts escaped spaces (`\ `) to actual spaces
fn sanitize_path(raw: &str) -> String {
    raw.trim()
        .trim_matches('"')
        .trim_matches('\'')
        .replace(r"\ ", " ")
}
