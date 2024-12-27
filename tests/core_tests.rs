use std::fs;
use std::path::Path;
use shellmorph_core::{encrypt_file, execute_shellcode};

#[test]
fn test_encrypt_decrypt_file() {
    let input_content = b"Test data for encryption and decryption.";
    let key = b"complexkey";
    let temp_input = Path::new("test_input.txt");
    let temp_output = Path::new("test_output.enc");
    let temp_decrypted = Path::new("test_decrypted.txt");

    // Write input content to a temp file
    fs::write(temp_input, input_content).unwrap();

    // Encrypt the file
    encrypt_file(temp_input, temp_output, key).unwrap();

    // Decrypt the file (XOR again with the same key)
    encrypt_file(temp_output, temp_decrypted, key).unwrap();

    // Verify decrypted content matches original content
    let decrypted_content = fs::read(temp_decrypted).unwrap();
    assert_eq!(decrypted_content, input_content);

    // Clean up
    fs::remove_file(temp_input).unwrap();
    fs::remove_file(temp_output).unwrap();
    fs::remove_file(temp_decrypted).unwrap();
}

#[test]
fn test_execute_shellcode_valid() {
    // Simple shellcode: exit(42)
    #[cfg(target_os = "linux")]
    let shellcode = b"\xB8\x2A\x00\x00\x00\xCD\x80"; // Linux x86 exit syscall

    let result = execute_shellcode(shellcode);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "Memory allocation failed")]
fn test_execute_shellcode_invalid() {
    // Invalid shellcode (too short)
    let shellcode = b"\x90";
    execute_shellcode(shellcode).unwrap();
}
