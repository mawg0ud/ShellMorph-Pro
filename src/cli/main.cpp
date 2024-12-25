#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <filesystem>
#include "core_wrapper.hpp"

namespace fs = std::filesystem;

void print_usage() {
    std::cout << "Usage: shellmorph-cli [encrypt|exec] <args>\n"
              << "Commands:\n"
              << "  encrypt <input_file> <output_file> <key> - Encrypt a file\n"
              << "  exec <shellcode_file> - Execute shellcode\n";
}

std::vector<uint8_t> read_binary_file(const std::string& file_path) {
    std::ifstream file(file_path, std::ios::binary);
    if (!file) {
        throw std::runtime_error("Failed to open file: " + file_path);
    }

    return std::vector<uint8_t>(std::istreambuf_iterator<char>(file), {});
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        print_usage();
        return 1;
    }

    std::string command = argv[1];

    try {
        if (command == "encrypt") {
            if (argc != 5) {
                print_usage();
                return 1;
            }

            fs::path input_file = argv[2];
            fs::path output_file = argv[3];
            std::string key = argv[4];

            if (!fs::exists(input_file)) {
                throw std::runtime_error("Input file does not exist.");
            }

            encrypt_file(input_file.string(), output_file.string(), key);
            std::cout << "File encrypted successfully: " << output_file << '\n';
        } else if (command == "exec") {
            if (argc != 3) {
                print_usage();
                return 1;
            }

            std::string shellcode_file = argv[2];
            auto shellcode = read_binary_file(shellcode_file);

            execute_shellcode(shellcode);
        } else {
            print_usage();
            return 1;
        }
    } catch (const std::exception& ex) {
        std::cerr << "Error: " << ex.what() << '\n';
        return 1;
    }

    return 0;
}
