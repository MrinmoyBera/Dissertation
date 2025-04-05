use tfhe::prelude::*;  // Import necessary traits
use tfhe::{FheInt8, ClientKey};
use csv::{ReaderBuilder, WriterBuilder};
use base64::{engine::general_purpose::STANDARD, Engine as _};

const ENCRYPTED_CSV: &str = "encrypted.csv";
const DECRYPTED_CSV: &str = "decrypted.csv";
const TARGET_COLUMN: usize = 1;

pub fn decrypt_csv(keys: &ClientKey) {
    let mut reader = ReaderBuilder::new().has_headers(true).from_path(ENCRYPTED_CSV).expect("Failed to read CSV");
    let mut writer = WriterBuilder::new().from_path(DECRYPTED_CSV).expect("Failed to write CSV");

    for result in reader.records() {
        let record = result.expect("Failed to read record");
        let mut modified_record: Vec<String> = record.iter().map(|s| s.to_string()).collect();

        if let Some(value) = record.get(TARGET_COLUMN) {
            if let Ok(decoded_bytes) = STANDARD.decode(value) {
                let encrypted_value: FheInt8 = bincode::deserialize(&decoded_bytes).expect("Failed to deserialize");
                let decrypted_value: i8 = encrypted_value.decrypt(keys);
                modified_record[TARGET_COLUMN] = decrypted_value.to_string();
            }
        }
        writer.write_record(&modified_record).expect("Failed to write record");
    }

    writer.flush().expect("Failed to flush writer");
    println!("Decryption complete. Decrypted CSV saved to {}", DECRYPTED_CSV);
}
