# ⬡ Ferrum AV Engine

> A transparent, hash-based antivirus engine written in Rust — built to expose how malware detection actually works, not hide it behind a black box.

![CI](https://github.com/krishpranav/ferrum-av/actions/workflows/workflow.yml/badge.svg)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange?logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue)

---

## Overview

Most antivirus engines are closed-source black boxes. You feed them a file, they return a verdict, you have no idea why. Ferrum is different — every detection decision is traceable, every module has one job, and the entire codebase is readable by a human being.

**Two purposes:**

1. **Education** — understand how AV engines actually work under the hood.
2. **Red team research** — once the engine is built, we'll use it to demonstrate exactly how malware evades detection.

---

## Series Roadmap

| Episode | Feature                                            | Status      |
|---------|----------------------------------------------------|-------------|
| 1       | Hash-based detection (MD5, SHA-1, SHA-256)         | ✅ Complete |
| 2       | YARA rule scanning                                 | 🔜 Planned  |
| 3       | String & API heuristics                            | 🔜 Planned  |
| 4       | PE header & section analysis                       | 🔜 Planned  |
| 5       | Entropy analysis (packed/encrypted file detection) | 🔜 Planned  |
| 6       | Fuzzy hashing (ssdeep/TLSH)                        | 🔜 Planned  |
| 7       | Malware evasion — defeating the engine             | 🔜 Planned  |

---
## Getting Started

### Prerequisites

- Rust 1.75+ — install via [rustup](https://rustup.rs)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Clone & Build

```bash
git clone https://github.com/YOUR_USERNAME/ferrum-av.git
cd ferrum-av

# Debug build
cargo build

# Optimised release binary (~1.5 MB, stripped)
cargo build --release
```

---

## Usage

### Scan a single file

```bash
./target/release/ferrum scan --file /path/to/suspicious.exe
```

### Scan a directory (recursive)

```bash
./target/release/ferrum scan --dir /path/to/samples/
```

### Add a hash to the signature database

```bash
./target/release/ferrum add-hash \
    --hash 8d3f68b16f0710f858d8c1d2c699260e6f43161a5510abb0e7ba567bd72c965b \
    --name "Ryuk Ransomware"
```

### Print database stats

```bash
./target/release/ferrum stats
```

### Use a custom signature database path

```bash
./target/release/ferrum scan --file sample.exe --sigdb /custom/path/sigs.txt
```

---

## Example Output

```
  ⬡ Ferrum AV v0.1.0

  Database:  47 signatures loaded

  Scanning: wannacry.exe

  DETECTED wannacry.exe
           SHA256 : ed01ebfbc9eb5bbea545af4d01bf5f107166...
           Threat : WannaCry Ransomware

  1 scanned — 1 threat(s) detected
```

---

## Signature Database

Hashes are stored in `signatures/signatures.txt` using a simple pipe-delimited format:

```
# Comment lines are ignored
SHA256|ed01ebfbc9eb5bbea545af4d01bf5f1071661840480439c6e5babe8e080e41aa|WannaCry Ransomware
MD5|84c82835a5d21bbcf75a61706d8ab549|WannaCry Ransomware
SHA1|4da1f312a214c07143abeeafb695d904440a420a|WannaCry Ransomware
```

Supported hash types: `MD5` (32 chars), `SHA1` (40 chars), `SHA256` (64 chars).

Populate your database with hashes from:

- [MalwareBazaar](https://bazaar.abuse.ch/)
- [VirusTotal](https://www.virustotal.com/)
- [Hybrid Analysis](https://www.hybrid-analysis.com/)

---

## Running the CI Locally

```bash
# Format check
cargo fmt --all -- --check

# Linter (strict)
cargo clippy --all-targets --all-features -- -D warnings

# Tests
cargo test --verbose
```

---

## Contributing

1. Fork the repo
2. Create a feature branch: `git checkout -b feat/yara-scanning`
3. Commit your changes: `git commit -m 'feat: add YARA rule scanning'`
4. Push and open a PR against `main`

All PRs must pass `fmt`, `clippy`, and `cargo test` before review.

---

## Disclaimer

This project is intended for **educational and research purposes only**. Do not use this tool on systems or files you do not own or have explicit permission to analyze. All malware samples referenced in this project are handled in isolated lab environments.

---

## License

MIT — see [LICENSE](LICENSE) for details.