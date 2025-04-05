use tfhe::prelude::*;
use csv::ReaderBuilder;
use tfhe::{FheInt8, ClientKey, ServerKey, set_server_key};
use base64::{engine::general_purpose::STANDARD, Engine as _};

const ENCRYPTED_CSV: &str = "encrypted.csv";
const TARGET_COLUMN: usize = 1;

pub fn compare_encrypted_values(keys: &ClientKey, server_key: &ServerKey, reference_value: i8) {
    set_server_key(server_key.clone());

    let mut reader = ReaderBuilder::new().has_headers(true).from_path(ENCRYPTED_CSV)
        .expect("Failed to read CSV");

    let reference_cipher = FheInt8::encrypt(reference_value, keys);

    println!("Performing encrypted comparisons with reference value: {}", reference_value);

    for result in reader.records() {
        let record = result.expect("Failed to read record");

        if let Some(encrypted_str) = record.get(TARGET_COLUMN) {
            if let Ok(decoded_bytes) = STANDARD.decode(encrypted_str) {
                let encrypted_value: FheInt8 = bincode::deserialize(&decoded_bytes).expect("Failed to deserialize");

                let gt = encrypted_value.gt(&reference_cipher);
                let eq = encrypted_value.eq(&reference_cipher);
                let lt = encrypted_value.lt(&reference_cipher);

                let dec_gt: bool = gt.decrypt(keys);
                let dec_eq: bool = eq.decrypt(keys);
                let dec_lt: bool = lt.decrypt(keys);

                println!(
                    "Encrypted Value: Comparisons -> (> {}: {}, == {}: {}, < {}: {})", 
                    reference_value, dec_gt, reference_value, dec_eq, reference_value, dec_lt
                );
            }
        }
    }
}
