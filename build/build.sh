#!/bin/bash

set -e  # Exit on error
set -o pipefail  # Capture pipeline errors

# Global Variables
BUILD_TYPE="Release"
CORE_DIR="src/core"
CLI_DIR="src/cli"
API_DIR="src/api"
OUTPUT_DIR="dist"

# Colors for pretty output
RED="\033[31m"
GREEN="\033[32m"
YELLOW="\033[33m"
RESET="\033[0m"

# Display help
function usage() {
    echo -e "${GREEN}ShellMorph Pro Build Script${RESET}"
    echo "Usage: $0 [options]"
    echo "Options:"
    echo "  -t, --type      Build type (Release/Debug)"
    echo "  -c, --clean     Clean build directory"
    echo "  -h, --help      Show this help message"
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -t|--type)
            BUILD_TYPE="$2"
            shift 2
            ;;
        -c|--clean)
            CLEAN=true
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${RESET}"
            usage
            exit 1
            ;;
    esac
done

# Clean build artifacts
if [[ "${CLEAN}" == true ]]; then
    echo -e "${YELLOW}Cleaning build directories...${RESET}"
    rm -rf "${OUTPUT_DIR}" "${CORE_DIR}/target" "${CLI_DIR}/build" "${API_DIR}/build"
    echo -e "${GREEN}Clean complete.${RESET}"
    exit 0
fi

# Step 1: Build the Rust core
echo -e "${YELLOW}Building Rust core...${RESET}"
cargo build --release --manifest-path "${CORE_DIR}/Cargo.toml"
mkdir -p "${OUTPUT_DIR}/core"
cp "${CORE_DIR}/target/release/libshellmorph_core.a" "${OUTPUT_DIR}/core"

# Step 2: Build the CLI
echo -e "${YELLOW}Building CLI...${RESET}"
cmake -S "${CLI_DIR}" -B "${CLI_DIR}/build" -DCMAKE_BUILD_TYPE="${BUILD_TYPE}"
cmake --build "${CLI_DIR}/build" --target shellmorph-cli
mkdir -p "${OUTPUT_DIR}/cli"
cp "${CLI_DIR}/build/shellmorph-cli" "${OUTPUT_DIR}/cli"

# Step 3: Build the API
echo -e "${YELLOW}Building API...${RESET}"
npm install --prefix "${API_DIR}"
npm run build --prefix "${API_DIR}"
mkdir -p "${OUTPUT_DIR}/api"
cp -r "${API_DIR}/build"/* "${OUTPUT_DIR}/api"

# Build Summary
echo -e "${GREEN}Build completed successfully!${RESET}"
echo "Output directory: ${OUTPUT_DIR}"
