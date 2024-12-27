#include <gtest/gtest.h>
#include <fstream>
#include <filesystem>
#include <cstdlib>

namespace fs = std::filesystem;

// Utility functions for file operations
std::string read_file(const std::string& path) {
    std::ifstream file(path, std::ios::binary);
    return std::string((std::istreambuf_iterator<char>(file)), std::istreambuf_iterator<char>());
}

void write_file(const std::string& path, const std::string& content) {
    std::ofstream file(path, std::ios::binary);
    file << content;
}

// Test cases
TEST(CliTests, EncryptAndDecryptFile) {
    std::string input_file = "test_input.txt";
    std::string encrypted_file = "test_output.enc";
    std::string decrypted_file = "test_decrypted.txt";
    std::string key = "mycomplexkey";

    std::string input_data = "This is some test data.";
    write_file(input_file, input_data);

    // Encrypt the file
    std::string encrypt_command = "./shellmorph-cli encrypt " + input_file + " " + encrypted_file + " " + key;
    ASSERT_EQ(std::system(encrypt_command.c_str()), 0);

    // Decrypt the file (re-encrypt with the same key)
    std::string decrypt_command = "./shellmorph-cli encrypt " + encrypted_file + " " + decrypted_file + " " + key;
    ASSERT_EQ(std::system(decrypt_command.c_str()), 0);

    // Validate decrypted content
    std::string decrypted_data = read_file(decrypted_file);
    EXPECT_EQ(decrypted_data, input_data);

    // Cleanup
    fs::remove(input_file);
    fs::remove(encrypted_file);
    fs::remove(decrypted_file);
}

TEST(CliTests, ExecuteShellcode) {
    std::string shellcode_file = "test_shellcode.bin";
    std::ofstream shellcode(shellcode_file, std::ios::binary);
    shellcode << '\xB8' << '\x2A' << '\x00' << '\x00' << '\x00' << '\xCD' << '\x80'; // exit(42) shellcode
    shellcode.close();

    std::string command = "./shellmorph-cli exec " + shellcode_file;
    ASSERT_EQ(std::system(command.c_str()), 0);

    fs::remove(shellcode_file);
}
