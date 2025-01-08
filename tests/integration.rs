use std::fs;
use std::path::Path;
use shellmorph_core::{encrypt_file, decrypt_file};

#[test]
fn test_end_to_end_encryption() {
    let input_file = "test_input.txt";
    let encrypted_file = "test_output.enc";
    let decrypted_file = "test_output.txt";
    let key = b"complexsecurekey";

    // Write test data
    fs::write(input_file, "Sensitive Data").expect("Failed to write input file");

    // Encrypt the file
    encrypt_file(input_file.into(), encrypted_file.into(), key)
        .expect("Encryption failed");

    // Decrypt the file
    decrypt_file(encrypted_file.into(), decrypted_file.into(), key)
        .expect("Decryption failed");

    // Validate the content
    let original_content = fs::read_to_string(input_file).unwrap();
    let decrypted_content = fs::read_to_string(decrypted_file).unwrap();
    assert_eq!(original_content, decrypted_content);

    // Cleanup
    fs::remove_file(input_file).unwrap();
    fs::remove_file(encrypted_file).unwrap();
    fs::remove_file(decrypted_file).unwrap();
}
