use windows_sys::Win32::Security::Cryptography::{CryptUnprotectData, CRYPT_INTEGER_BLOB};
use windows_sys::Win32::Foundation::LocalFree;
use std::ptr::null_mut;
use std::fs;
use base64::prelude::*;
use serde_json::Value;

// Define a type alias for DATA_BLOB as CRYPT_INTEGER_BLOB, following Rust's naming convention
type DataBlob = CRYPT_INTEGER_BLOB;

fn decrypt_data(encrypted_data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Create the input DataBlob struct
    let mut blob_in = DataBlob {
        cbData: encrypted_data.len() as u32,
        pbData: encrypted_data.as_ptr() as *mut u8,
    };

    // Create an empty output DataBlob struct
    let mut blob_out = DataBlob {
        cbData: 0,
        pbData: null_mut(),
    };

    // Decrypt the data using CryptUnprotectData
    let result = unsafe {
        CryptUnprotectData(
            &mut blob_in,
            null_mut(),   // description
            null_mut(),   // entropy
            null_mut(),   // reserved
            null_mut(),   // prompt struct
            0,            // flags
            &mut blob_out
        )
    };

    if result != 0 {
        let decrypted_data = unsafe {
            std::slice::from_raw_parts(blob_out.pbData, blob_out.cbData as usize).to_vec()
        };

        // Free the memory allocated for the output blob
        unsafe { LocalFree(blob_out.pbData as *mut _); }

        Ok(decrypted_data)
    } else {
        Err("Failed to decrypt data".into())
    }
}

fn get_secret_key(chrome_path_local_state: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string(chrome_path_local_state)?;
    let v: Value = serde_json::from_str(&file_content)?;

    // Get the encrypted key in base64 from the Local State file
    let key_base64 = v["os_crypt"]["encrypted_key"]
        .as_str()
        .ok_or("No key found")?;
    let mut key_bytes = BASE64_STANDARD.decode(key_base64)?;

    // Remove the DPAPI prefix (first 5 bytes)
    key_bytes.drain(0..5);

    // Decrypt the key bytes
    let decrypted_key = decrypt_data(&key_bytes)?;
    let key_hex = hex::encode(decrypted_key);
    Ok(key_hex)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chrome_local_state_path = "path_to_local_state";  // Update this path to your actual local state file
    match get_secret_key(chrome_local_state_path) {
        Ok(secret_key) => println!("Decrypted key: {}", secret_key),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
