# RustChef 🦀

A CyberChef-inspired command-line data transformation tool built in Rust.

**Author:** Cheikh Melainine — 22027 | SUPNUM Master Cybersécurité 2025-2026

---

## Features — 23 Operations

| Category | Operations |
|----------|-----------|
| Encoding | base64-encode, base64-decode, hex-encode, hex-decode, url-encode, url-decode |
| Text | rot13, binary-encode, binary-decode, reverse, to-upper, to-lower |
| Hashing | md5, sha1, sha256 |
| Crypto | xor, xor-decode |
| Extraction | extract-ip, extract-url, extract-email, extract-domain |
| Analysis | stats, entropy, char-freq |

---

## Installation

### From source (requires Rust 1.70+)
```bash
git clone https://github.com/supnum/rustchef
cd rustchef
cargo build --release
./target/release/rustchef --help
```

### Via Docker
```bash
docker build -t rustchef .
docker run --rm rustchef --help
docker run --rm rustchef base64-encode --input "hello"
echo "hello world" | docker run --rm -i rustchef sha256
```

---

## Usage

```
rustchef <OPERATION> [--input <TEXT>]
```

Input is read from `--input` flag **or** from stdin if omitted.

---

## Examples

### Encoding / Decoding
```bash
# Base64
rustchef base64-encode --input "Hello, World!"
# SGVsbG8sIFdvcmxkIQ==

rustchef base64-decode --input "SGVsbG8sIFdvcmxkIQ=="
# Hello, World!

# Hex
rustchef hex-encode --input "RustChef"
# 5275737443686566

rustchef hex-decode --input "5275737443686566"
# RustChef

# URL encoding
rustchef url-encode --input "hello world & foo=bar"
# hello%20world%20%26%20foo%3Dbar

# ROT13
rustchef rot13 --input "Hello World"
# Uryyb Jbeyq

# Binary
rustchef binary-encode --input "Hi"
# 01001000 01101001

rustchef binary-decode --input "01001000 01101001"
# Hi

# Reverse
rustchef reverse --input "RustChef"
# fehCdsuR

# Case
rustchef to-upper --input "hello"   # HELLO
rustchef to-lower --input "WORLD"   # world
```

### Hashing
```bash
rustchef md5    --input "hello"
# MD5("hello") = 5d41402abc4b2a76b9719d911017c592

rustchef sha1   --input "hello"
# SHA1("hello") = aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d

rustchef sha256 --input "hello"
# SHA256("hello") = 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
```

### XOR Cipher
```bash
rustchef xor --input "Hello" --key "secret"
# 3a1d1f0a0b

rustchef xor-decode --input "3a1d1f0a0b" --key "secret"
# Hello
```

### Extraction
```bash
# From file via stdin
rustchef extract-ip    < samples/sample.txt
rustchef extract-url   < samples/sample.txt
rustchef extract-email < samples/sample.txt

# Inline
rustchef extract-ip --input "Hosts: 192.168.0.1 and 10.0.0.1"
# 192.168.0.1
# 10.0.0.1
# [2 match(es) found]
```

### Analysis
```bash
rustchef stats --input "Hello World"
# Characters : 11
# Words      : 2
# Lines      : 1
# Unique chars: 9
# Most common: "l" (3 times)

rustchef entropy --input "hello world"
# Entropy: 3.2776 bits/byte
# Assessment: Low entropy (structured/text)

rustchef entropy --input "$(cat /dev/urandom | head -c 100 | base64)"
# Entropy: ~6.0+ bits/byte
# Assessment: High entropy (random/encrypted/compressed)

rustchef char-freq --input "hello world" --top 5
# Char         Count  Frequency
# --------------------------------
# l                3     27.27%
# o                2     18.18%
# ...
```

### Piping (stdin support)
```bash
echo "hello" | rustchef base64-encode
cat samples/sample.txt | rustchef extract-email
echo "aGVsbG8=" | rustchef base64-decode | rustchef sha256
```

---

## Running Tests

```bash
# Unit tests (all modules)
cargo test

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test integration

# Linting
cargo clippy -- -D warnings
```

---

## Docker

```bash
# Build
docker build -t rustchef .

# Run any operation
docker run --rm rustchef sha256 --input "hello"
docker run --rm rustchef rot13 --input "Hello World"

# Pipe from host
echo "192.168.1.1 and 10.0.0.1" | docker run --rm -i rustchef extract-ip

# From file
docker run --rm -i rustchef extract-url < samples/sample.txt
```

---

## Architecture

```
rustchef/
├── src/
│   ├── main.rs           # CLI entry point (clap subcommands)
│   └── ops/
│       ├── mod.rs        # Module declarations
│       ├── encoding.rs   # Base64, Hex, URL, ROT13, Binary, Reverse, Case
│       ├── hashing.rs    # MD5, SHA1, SHA256
│       ├── crypto.rs     # XOR cipher
│       ├── extraction.rs # Regex-based IP/URL/Email/Domain extraction
│       └── analysis.rs   # Text stats, Shannon entropy, char frequency
├── tests/
│   └── integration.rs    # CLI integration tests (assert_cmd)
├── samples/
│   └── sample.txt        # Sample input file
├── Cargo.toml
├── Dockerfile
└── README.md
```

**Dependencies:**
- `clap 4` — CLI argument parsing with derive macros
- `base64 0.22` — Base64 encoding/decoding
- `hex 0.4` — Hex encoding/decoding
- `urlencoding 2.1` — URL percent-encoding
- `md-5 0.10`, `sha1 0.10`, `sha2 0.10`, `digest 0.10` — Cryptographic hashing
- `regex 1` — Pattern matching for extraction operations

---

## Limitations

- XOR output is hex-encoded (not raw bytes) for terminal safety
- Binary decode requires space-separated 8-bit groups
- Extraction uses heuristic regex patterns (may miss edge cases)
- No streaming mode for very large files
- No chaining of operations (each command is independent)

---

## License

MIT — Cheikh Melainine, SUPNUM 2025-2026
