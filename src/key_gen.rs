use tfhe::{generate_keys, set_server_key, ConfigBuilder, ClientKey, ServerKey};
use std::fs::File;
use std::io::{Read, Write};
use bincode;

const KEY_FILE: &str = "keyfile.bin";

/// Save data to a file
fn save_to_file(filename: &str, data: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(data)?;
    Ok(())
}

/// Load data from a file
fn load_from_file(filename: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

/// Generate or load TFHE keys
pub fn initialize_keys() -> (ClientKey, ServerKey) {
    if let Ok(key_data) = load_from_file(KEY_FILE) {
        let keys: ClientKey = bincode::deserialize(&key_data).expect("Failed to deserialize keys");
        let config = ConfigBuilder::default().build();
        let (_, server_keys) = generate_keys(config);
        set_server_key(server_keys.clone());
        println!("Keys loaded from file.");
        (keys, server_keys)
    } else {
        let config = ConfigBuilder::default().build();
        let (keys, server_keys) = generate_keys(config);
        save_to_file(KEY_FILE, &bincode::serialize(&keys).unwrap()).expect("Failed to save keys");
        set_server_key(server_keys.clone());
        println!("New keys generated and saved.");
        (keys, server_keys)
    }
}
