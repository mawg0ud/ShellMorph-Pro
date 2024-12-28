use std::fs;
use std::path::Path;
use shellmorph_core::encrypt_file;

#[test]
fn stress_test_encrypt_large_file() {
    let temp_input = Path::new("large_input.txt");
    let temp_output = Path::new("large_output.enc");
    let key = b"supersecurekey";

    // Generate a large file (1 GB)
    let mut large_data = Vec::new();
    large_data.resize(1024 * 1024 * 1024, 0xAB); // 1 GB of 0xAB
    fs::write(temp_input, &large_data).unwrap();

    // Encrypt the large file
    encrypt_file(temp_input, temp_output, key).unwrap();

    // Verify output file size matches
    let metadata = fs::metadata(temp_output).unwrap();
    assert_eq!(metadata.len(), large_data.len() as u64);

    // Clean up
    fs::remove_file(temp_input).unwrap();
    fs::remove_file(temp_output).unwrap();
}
