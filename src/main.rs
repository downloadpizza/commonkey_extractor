use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::env;


fn main() -> std::io::Result<()> {
    // get the path from input
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path/to/otp_dump.bin", args[0]);
        std::process::exit(1);
    }
    let path = &args[1];

    // open file
    let mut file = File::open(path)?;

    // get common key offset (to be added)
    file.seek(SeekFrom::Start(0xE0))?;

    // read the first 16 bytes
    let mut key = [0xE0u8; 16];
    file.read_exact(&mut key)?;

    // print key in hex(adecimal)
    print!("Your Common Key is: ");
    for byte in &key {
        print!("{:02X}", byte);
    }
    println!();

    Ok(())
}
