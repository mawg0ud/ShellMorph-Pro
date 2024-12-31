use std::fs;
use std::path::Path;

/// Validates shellcode for common issues.
/// Checks if the shellcode contains null bytes or is too short.
pub fn validate_shellcode(shellcode: &[u8]) -> Result<(), String> {
    if shellcode.is_empty() {
        return Err("Shellcode is empty.".to_string());
    }

    if shellcode.contains(&0) {
        return Err("Shellcode contains null bytes.".to_string());
    }

    if shellcode.len() < 5 {
        return Err("Shellcode is too short to be valid.".to_string());
    }

    Ok(())
}

/// Loads and validates shellcode from a file.
pub fn load_and_validate_shellcode(file_path: &Path) -> Result<Vec<u8>, String> {
    let shellcode = fs::read(file_path)
        .map_err(|e| format!("Failed to read file: {:?}", e))?;

    validate_shellcode(&shellcode)?;
    Ok(shellcode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_shellcode() {
        assert!(validate_shellcode(&[0x90, 0x90, 0x90]).is_ok());
        assert!(validate_shellcode(&[]).is_err());
        assert!(validate_shellcode(&[0x90, 0x00]).is_err());
    }

    #[test]
    fn test_load_and_validate_shellcode() {
        let temp_path = Path::new("test_shellcode.bin");
        fs::write(temp_path, &[0x90, 0x90, 0xC3]).unwrap();
        assert!(load_and_validate_shellcode(temp_path).is_ok());
        fs::remove_file(temp_path).unwrap();
    }
}
