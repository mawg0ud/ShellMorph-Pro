# Getting Started with ShellMorph Pro

This tutorial guides you through setting up and using ShellMorph Pro for the first time.

---

## Prerequisites
1. Rust (`1.70.0+`)
2. Node.js (`20.x.x+`)
3. .NET SDK (`7.0+`)

### Install Dependencies
- On **Linux/macOS**:
  ```bash
  sudo apt-get install -y rustc cargo nodejs dotnet-sdk
  ```

- On **Windows**:
  1. Install Rust via [rustup.rs](https://rustup.rs/).
  2. Install Node.js via [nodejs.org](https://nodejs.org/).
  3. Install .NET SDK from [Microsoft](https://dotnet.microsoft.com/).

---

## Step 1: Clone the Repository
```bash
git clone https://github.com/mawg0ud/ShellMorph-Pro.git
cd ShellMorphPro
```

---

## Step 2: Build the Project

### Build the Rust Core
```bash
cargo build --release
```

### Install Node.js Dependencies
```bash
cd src/api
npm install
```

### Build the GUI (Optional)
```bash
cd src/gui
dotnet build
```

---

## Step 3: Run the Project

### Start the API Server
```bash
node src/api/api_server.js
```

### Test the API
Use a tool like [Postman](https://www.postman.com/) or the provided CLI:
```bash
curl -X POST -H "Authorization: Bearer <API_KEY>" \
     -d '{"file_path": "test.exe", "architecture": "x64"}' \
     http://localhost:8080/api/v1/generate
```

### Launch the GUI
```bash
dotnet run --project src/gui
```

---

Congratulations! You’ve successfully set up ShellMorph Pro. Explore more advanced tutorials in the `docs/tutorials/` directory.
