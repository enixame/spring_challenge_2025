# 🧠 Rust DFS Board Exploration - spring_challenge_2025

This project implements a depth-first search (DFS) with memoization for exploring 3x3 board states.

## 📦 Requirements

- [Rust toolchain](https://www.rust-lang.org/tools/install)

To install Rust:

### ▶ On Unix (Linux/macOS)

```bash
curl https://sh.rustup.rs -sSf | sh
```

### ▶ On Windows (PowerShell)

```bash
irm https://sh.rustup.rs -UseBasicParsing | iex
```

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
