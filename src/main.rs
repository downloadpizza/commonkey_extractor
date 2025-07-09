use std::fs::File;
use std::io::{self, Read, Write, Seek};
use std::path::Path;
use std::{thread, time::Duration};
use colored::*;
use std::process::Command;

const COMMON_KEY_OFFSET: u64 = 0xE0;
const COMMON_KEY_SIZE: usize = 16;

fn clear_screen() {
    if cfg!(target_os = "windows") {
        // Windows
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        // Linux and macOS
        Command::new("clear").status().unwrap();
    }
}

fn main() -> io::Result<()> {
    loop {
        clear_screen();
        println!("Where is your OTP path?");
        println!("You can drag and drop it in Finder / File Explorer.");
        print!("> ");
        io::stdout().flush()?; // flush prompt

        let mut path = String::new();
        io::stdin().read_line(&mut path)?;
        let path = path.trim().trim_matches(['\'', '"'].as_ref());

        if !Path::new(&path).exists() {
            eprintln!(
                "{}",
                "ERROR! Path does not exist. Did you misspell something? Trying again in 5 seconds..."
                    .red()
            );
            thread::sleep(Duration::from_secs(5));
            continue;
        }

        // Open OTP and read Common Key
        let mut file = File::open(path)?;
        file.seek(io::SeekFrom::Start(COMMON_KEY_OFFSET))?;

        let mut key = [0u8; COMMON_KEY_SIZE];
        file.read_exact(&mut key)?;

        println!("\nWii U Common Key:");
        for byte in &key {
            print!("{:02X}", byte);
        }
        println!("\n");

        println!("Press Ctrl+C to quit...");

        // Wait forever until Ctrl+C
        loop {
            thread::sleep(Duration::from_secs(1));
        }
    }
}
