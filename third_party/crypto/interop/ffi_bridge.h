#pragma once

#include <string>
#include <vector>

extern "C" {
    // Rust FFI methods
    void rust_encrypt_file(const char* input_path, const char* output_path, const char* key);
    void rust_execute_shellcode(const uint8_t* shellcode, size_t shellcode_size);
}

class RustBridge {
public:
    static void encryptFile(const std::string& inputPath, const std::string& outputPath, const std::string& key) {
        rust_encrypt_file(inputPath.c_str(), outputPath.c_str(), key.c_str());
    }

    static void executeShellcode(const std::vector<uint8_t>& shellcode) {
        rust_execute_shellcode(shellcode.data(), shellcode.size());
    }
};
