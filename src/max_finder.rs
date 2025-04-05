use tfhe::prelude::*;
use tfhe::{set_server_key, ConfigBuilder, FheUint8, ClientKey, ServerKey};
use csv::ReaderBuilder;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use bincode;
use std::error::Error;

const ENCRYPTED_CSV: &str = "encrypted.csv";  
const TARGET_COLUMN: usize = 1;  // Column to process

/// Finds the maximum encrypted value from a CSV column and returns it
pub fn find_max_encrypted(keys: &ClientKey) -> Result<FheUint8, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().has_headers(true).from_path(ENCRYPTED_CSV)?;
    let mut max_encrypted: Option<FheUint8> = None;

    for result in reader.records() {
        let record = result?;
        if let Some(encrypted_str) = record.get(TARGET_COLUMN) {
            if let Ok(decoded_bytes) = STANDARD.decode(encrypted_str) {
                let encrypted_value: FheUint8 = bincode::deserialize(&decoded_bytes)?;

                // Compare encrypted values
                max_encrypted = match max_encrypted {
                    Some(current_max) => Some(current_max.max(&encrypted_value)),
                    None => Some(encrypted_value),
                };
            }
        }
    }

    max_encrypted.ok_or_else(|| "No valid encrypted values found".into())
}
