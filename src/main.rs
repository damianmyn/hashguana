use std::{fs::File, io::{self, stdin, stdout, Read, Write}};
use sha2::{Sha256, Digest};

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




fn main() {
    print!("Input file path: ");
    stdout().flush();
    let mut input_string: String     = String::new();

    stdin().read_line(&mut input_string);

    let filepath = input_string.trim(); 

    match get_file_hash(filepath) {
        Ok(hash) => println!("SHA256 | {}", hash),
        Err(e) => eprint!("Error computing hash: {}", e),
    };
}
