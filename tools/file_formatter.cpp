#include <iostream>
#include <fstream>
#include <iomanip>
#include <vector>

class FileFormatter {
public:
    static void formatFile(const std::string& inputFilePath, const std::string& outputFilePath) {
        std::ifstream inputFile(inputFilePath, std::ios::binary);
        std::ofstream outputFile(outputFilePath);

        if (!inputFile.is_open()) {
            throw std::runtime_error("Unable to open input file: " + inputFilePath);
        }

        if (!outputFile.is_open()) {
            throw std::runtime_error("Unable to open output file: " + outputFilePath);
        }

        std::vector<unsigned char> buffer(std::istreambuf_iterator<char>(inputFile), {});
        size_t bytesPerLine = 16;

        for (size_t i = 0; i < buffer.size(); i += bytesPerLine) {
            outputFile << std::setw(8) << std::setfill('0') << std::hex << i << ": ";

            for (size_t j = 0; j < bytesPerLine && i + j < buffer.size(); ++j) {
                outputFile << std::setw(2) << static_cast<int>(buffer[i + j]) << " ";
            }

            outputFile << "\n";
        }

        inputFile.close();
        outputFile.close();
    }
};

int main(int argc, char* argv[]) {
    if (argc != 3) {
        std::cerr << "Usage: file_formatter <input file> <output file>\n";
        return 1;
    }

    try {
        FileFormatter::formatFile(argv[1], argv[2]);
        std::cout << "File formatting completed successfully.\n";
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << "\n";
        return 1;
    }

    return 0;
}
