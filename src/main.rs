use std::{fs::File, io::{self, stdin, stdout, Read, Write}, path::Path};
use sha2::{Sha256, Digest};
use colored::Colorize;

fn get_file_hash(filepath: &str) -> io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    let hash_result = hasher.finalize();
    Ok(format!("{:x}", hash_result))
}

fn is_valid_hash(input: &str) -> bool {
    input.len() == 64 && input.chars().all(|c| c.is_ascii_hexdigit())
}

fn main() {
    print!("Input file path: ");
    let _ = stdout().flush();
    let mut input_string = String::new();
    let _ = stdin().read_line(&mut input_string);
    let filepath = input_string.trim();

    let original_hash = match get_file_hash(filepath) {
        Ok(hash) => {
            println!("SHA256 | {}", hash);
            hash
        }
        Err(e) => {
            eprintln!("Error computing hash: {}", e);
            return;
        }
    };

    print!("Input hash (file or paste from clipboard): ");
    let _ = stdout().flush();
    let mut compare_input = String::new();
    let _ = stdin().read_line(&mut compare_input);
    let compare_input = compare_input.trim();

    let compare_hash = if Path::new(compare_input).exists() {
        println!("Detected as file path, computing hash...");
        match get_file_hash(compare_input) {
            Ok(hash) => {
                println!("SHA256 | {}", hash);
                hash
            }
            Err(e) => {
                eprintln!("Error computing hash: {}", e);
                return;
            }
        }
    } else if is_valid_hash(compare_input) {
        println!("Detected as hash string");
        compare_input.to_string()
    } else {
        eprintln!("Invalid input: not a valid file path or SHA256 hash");
        return;
    };

    println!("\n--- Comparison Result ---");
    if original_hash == compare_hash {
        println!("{}", "✓ Hashes MATCH".green());
    } else {
        println!("✗ Hashes DO NOT MATCH");
        println!("Original: {}", original_hash.green());
        println!("Compare:  {}", compare_hash.red());
    }
}