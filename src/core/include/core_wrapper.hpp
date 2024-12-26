#pragma once
#include <string>

// Encrypt a file using the Rust core library
void encrypt_file(const std::string& input_path, const std::string& output_path, const std::string& key);

// Execute shellcode using the Rust core library
void execute_shellcode(const std::vector<uint8_t>& shellcode);
