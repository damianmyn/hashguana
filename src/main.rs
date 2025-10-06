use std::{fs::File, io::{self, stdin, stdout, Read, Write}, path::Path, process::Command};
use sha2::{Sha256, Digest};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

fn get_file_hash(filepath: &str) -> io::Result<String> {
    let mut file = File::open(filepath)?;
    let file_size = file.metadata()?.len();
    
    let pb = ProgressBar::new(file_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );
    
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];
    let mut total_read = 0u64;
    
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
        total_read += bytes_read as u64;
        pb.set_position(total_read);
    }
    
    pb.finish_with_message("Complete");
    
    let hash_result = hasher.finalize();
    Ok(format!("{:x}", hash_result))
}

fn is_valid_hash(input: &str) -> bool {
    input.len() == 64 && input.chars().all(|c| c.is_ascii_hexdigit())
}

fn is_sig_file(filepath: &str) -> bool {
    filepath.ends_with(".sig") || filepath.ends_with(".asc")
}

fn verify_signature(file_path: &str, sig_path: &str) -> io::Result<bool> {
    println!("\n{}", "--- GPG Signature Verification ---".cyan());
    println!("File: {}", file_path);
    println!("Signature: {}", sig_path);
    println!("Verifying...\n");
    
    let output = Command::new("gpg")
        .arg("--verify")
        .arg(sig_path)
        .arg(file_path)
        .output()?;
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    print!("{}", stderr);
    
    Ok(output.status.success())
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

    print!("Input hash/signature file (file path or paste hash): ");
    let _ = stdout().flush();
    let mut compare_input = String::new();
    let _ = stdin().read_line(&mut compare_input);
    let compare_input = compare_input.trim();

    if Path::new(compare_input).exists() && is_sig_file(compare_input) {
        println!("Detected as signature file");
        match verify_signature(filepath, compare_input) {
            Ok(true) => {
                println!("\n{}", "✓ Signature verification SUCCESSFUL".green().bold());
                println!("The file is authentic and has not been tampered with.");
            }
            Ok(false) => {
                println!("\n{}", "✗ Signature verification FAILED".red().bold());
                println!("The file may have been modified or the signature is invalid.");
            }
            Err(e) => {
                eprintln!("\n{}", "✗ Error during signature verification".red());
                eprintln!("Error: {}", e);
                eprintln!("\nMake sure GPG is installed on your system:");
                eprintln!("  - Linux: sudo apt install gnupg");
                eprintln!("  - macOS: brew install gnupg");
                eprintln!("  - Windows: https://www.gnupg.org/download/");
            }
        }
        return;
    }

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
        eprintln!("Invalid input: not a valid file path, signature file, or SHA256 hash");
        return;
    };

    println!("\n--- Hash Comparison Result ---");
    if original_hash == compare_hash {
        println!("{}", "✓ Hashes MATCH".green());
    } else {
        println!("✗ Hashes DO NOT MATCH");
        println!("Original: {}", original_hash.green());
        println!("Compare:  {}", compare_hash.red());
    }
}