mod key_gen;
mod encrypting_csv;
mod decrypt;
mod max_finder;
use tfhe::set_server_key;
use tfhe::prelude::*;
fn main() {
    // Step 1: Initialize Keys
    let (keys, server_key) = key_gen::initialize_keys();
    set_server_key(server_key.clone());

    // Step 2: Encrypt CSV File
    encrypting_csv::encrypt_csv(&keys);

    // Step 3: Decrypt CSV File (optional for debugging)
    decrypt::decrypt_csv(&keys);

    // Step 4: Find Maximum Encrypted Value in CSV
    match max_finder::find_max_encrypted(&keys) {
        Ok(encrypted_max) => {
            let decrypted_max: u8 = encrypted_max.decrypt(&keys);
            println!("Decrypted max value: {}", decrypted_max);
        }
        Err(e) => {
            eprintln!("Error finding max value: {}", e);
        }
    }
}
