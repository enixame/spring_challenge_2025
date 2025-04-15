# 🧠 Rust DFS Board Exploration - spring_challenge_2025

This project implements a depth-first search (DFS) with memoization for exploring 3x3 board states.


## 📦 Requirements

- [Rust toolchain](https://www.rust-lang.org/tools/install)

To install Rust:

### ▶ On Unix (Linux)

```bash
curl https://sh.rustup.rs -sSf | sh
```

### ▶ On Windows (PowerShell)

```bash
Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
.\rustup-init.exe
```


## 🔧 VS Code Setup (Optional but Recommended)

You can use Visual Studio Code for development.

### ▶ Install VS Code

Download and install VS Code from: [Visual Studio Code](https://code.visualstudio.com/)

### ▶ On Unix (Linux)

🐧 On Unix (Debian-based systems)

```bash
sudo apt update
sudo apt install wget gpg
wget -qO- https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor > packages.microsoft.gpg
sudo install -o root -g root -m 644 packages.microsoft.gpg /etc/apt/trusted.gpg.d/
sudo sh -c 'echo "deb [arch=amd64] https://packages.microsoft.com/repos/vscode stable main" > /etc/apt/sources.list.d/vscode.list'
sudo apt update
sudo apt install code
```

If you're using Fedora, Arch, etc., refer to: [Install VS Code on Linux](https://code.visualstudio.com/docs/setup/linux)

### ▶ On Windows (PowerShell)

```bash
Invoke-WebRequest -Uri https://aka.ms/win32-x64-user-stable -OutFile VSCodeSetup.exe
Start-Process .\VSCodeSetup.exe
```
This downloads and launches the official installer for the stable build.

### ▶ Recommended Extensions

Install the following extensions from the VS Code Marketplace:

- rust-analyzer (official language server)

- CodeLLDB (debugger support)

- Dependi (manage dependencies and address vulnerabilities in Rust)

- Even Better TOML (syntax support for .toml files)

You can find these in the Extensions panel (Ctrl+Shift+X) or by visiting the [VS Code marketplace](https://marketplace.visualstudio.com/).


## ⚙️ Requirements

### ▶ Compile in release mode

```bash
cargo build --release
```

### ▶ Run the program with input from file

Prepare an input file, for example: input.txt

```bash
5
0 0 0
0 0 0
0 0 0
123456789
```

This means:

- Line 1: depth
- Line 2-4: board
- Line 5: expected result (for test or comparison)

### ▶ Run on Unix/Linux/macOS

```bash
cargo run --release < input.txt
```

Or directly with the compiled binary:

```bash
./target/release/spring_challenge_2025 < input.txt
```

### ▶ Run on Windows PowerShell

PowerShell does not support < input.txt, so use:

```bash
Get-Content input.txt | cargo run --release
```

Or with the compiled binary:

```bash
Get-Content input.txt | .\target\release\spring_challenge_2025.exe
```


## 🧪 Run Tests

### ▶ Standard tests (unit + file-based)

```bash
cargo test -- --nocapture
```
The --nocapture flag shows println!() output during tests.


## 📂 Test Data Files

You can place test input files in a directory like tests/data/:
```bash
tests/data/
├── input1.txt
├── input2.txt
```

Each file must follow this structure:
```bash
<depth>
<row 1>
<row 2>
<row 3>
<expected result>
```
They are used by automated tests in main.rs.
