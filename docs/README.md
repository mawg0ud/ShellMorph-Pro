# ShellMorph Pro

ShellMorph Pro is a cutting-edge tool for transforming Portable Executables (PE) into shellcode, offering advanced injection techniques, modular architecture, and support for modern platforms and architectures.

## Key Features
- **High Performance:** Implemented using Rust for critical operations.
- **Cross-Platform:** Compatible with Windows, macOS, and Linux.
- **Advanced Injection Methods:** Supports Thread Hijacking, APC Injection, and Process Hollowing.
- **Extensibility:** Modular design with plugins for new architectures or injection methods.
- **User Interfaces:** Includes CLI, API, and optional GUI for diverse user requirements.

## Supported Architectures
- x86
- x64
- ARM64

## Components
- **Core:** Written in Rust for secure and fast operations.
- **GUI:** Developed in C# for Windows users.
- **Web Interface:** TypeScript-based dashboard with Node.js backend.

## Quick Start

### Prerequisites
1. Install Rust (`1.70.0` or later).
2. Install Node.js (`20.x.x` or later).
3. Install C# .NET SDK (`7.0` or later).

### Build and Run
1. Clone the repository:
   ```bash
   git clone https://github.com/mawg0ud/ShellMorph-Pro.git
   cd ShellMorphPro
   ```

2. Build the Rust core:
   ```bash
   cargo build --release
   ```

3. Start the API server:
   ```bash
   node src/api/api_server.js
   ```

4. (Optional) Launch the GUI:
   ```bash
   dotnet run --project src/gui
   ```

## Documentation
Detailed API and usage instructions can be found in the [`docs/`](.) directory.

## License
ShellMorph Pro is released under the MIT License.
```
For further assistance, refer to the [API Documentation](../API.md).
```
