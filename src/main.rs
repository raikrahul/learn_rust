use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    const ITEM_SIZE: usize = 1; // Size of each item (1 byte)
    const NUM_ITEMS: usize = 256; // Number of items to read
    let mut rec: Vec<u8> = vec![0; ITEM_SIZE * NUM_ITEMS]; // Buffer for NUM_ITEMS items

    // Check for exactly 3 arguments (program name + 3 args = 4)
    if args.len() != 4 {
        println!("Error: Program requires exactly 3 arguments");
        println!("Usage: {} <arg1> <arg2> <arg3>", args[0]);
        return;
    }

    // Use File::open for reading the source file
    let mut src = File::open(&args[1]);
    if src.is_err() {
        println!("Error opening source file");
        return;
    }
    let mut src = src.unwrap();

    // Use OpenOptions for write access to the target file
    let mut opt2 = OpenOptions::new();
    opt2.write(true).create(true);
    let mut dst = opt2.open(&args[2]);
    if dst.is_err() {
        println!("Error opening destination file");
        return;
    }
    let mut dst = dst.unwrap();

    println!("All 3 arguments provided:");
    println!("Argument 1: {}", args[1]);
    println!("Argument 2: {}", args[2]);
    println!("Argument 3: {}", args[3]);

    // Read and write loop
    loop {
        // Read NUM_ITEMS items, each of ITEM_SIZE bytes
        let bytes_read = src.read(&mut rec);

        if bytes_read.is_err() {
            println!("Error reading from source file");
            return;
        }
        let bytes_read = bytes_read.unwrap();

        if bytes_read == 0 {
            break; // End of file reached
        }

        // Process the bytes stored in rec
        println!("Bytes read: {:?}", &rec[0..bytes_read]);

        // Write the bytes to the destination
        let bytes_written = dst.write(&rec[0..bytes_read]);

        if bytes_written.is_err() {
            println!("Error writing to destination file");
            return;
        }
    }

    // Dropping files explicitly (though not necessary, Rust handles this)
    drop(src);
    drop(dst);

    println!("File copy completed successfully.");
}
