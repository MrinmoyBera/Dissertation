use csv::{ReaderBuilder, WriterBuilder};
use tfhe::{FheInt8, ClientKey};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use tfhe::prelude::*;  

const INPUT_CSV: &str = "/home/csc-pc4/TFHE_hadoop/sample_data.csv";  
const ENCRYPTED_CSV: &str = "encrypted.csv";  
const TARGET_COLUMN: usize = 1;  // Column to encrypt

pub fn encrypt_csv(keys: &ClientKey) {
    let mut reader = ReaderBuilder::new().has_headers(true).from_path(INPUT_CSV).expect("Failed to read CSV");
    let mut writer = WriterBuilder::new().from_path(ENCRYPTED_CSV).expect("Failed to write CSV");

    for result in reader.records() {
        let record = result.expect("Failed to read record");
        let mut modified_record: Vec<String> = record.iter().map(|s| s.to_string()).collect();

        if let Some(value) = record.get(TARGET_COLUMN) {
            if let Ok(parsed_value) = value.parse::<i8>() {
                let encrypted_value = FheInt8::try_encrypt(parsed_value, keys).expect("Encryption failed");
                let encrypted_bytes = bincode::serialize(&encrypted_value).expect("Serialization failed");
                modified_record[TARGET_COLUMN] = STANDARD.encode(encrypted_bytes);
            }
        }
        writer.write_record(&modified_record).expect("Failed to write record");
    }

    writer.flush().expect("Failed to flush writer");
    println!("Encryption complete. Encrypted CSV saved to {}", ENCRYPTED_CSV);
}
