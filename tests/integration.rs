use assert_cmd::Command;
use predicates::str::contains;

fn cmd() -> Command {
    Command::cargo_bin("rustchef").unwrap()
}

// ── Encoding ──────────────────────────────────────────────────────────────
#[test]
fn test_base64_encode_cli() {
    cmd().args(["base64-encode", "--input", "hello"])
        .assert().success().stdout(contains("aGVsbG8="));
}

#[test]
fn test_base64_decode_cli() {
    cmd().args(["base64-decode", "--input", "aGVsbG8="])
        .assert().success().stdout(contains("hello"));
}

#[test]
fn test_hex_encode_cli() {
    cmd().args(["hex-encode", "--input", "AB"])
        .assert().success().stdout(contains("4142"));
}

#[test]
fn test_hex_decode_cli() {
    cmd().args(["hex-decode", "--input", "4142"])
        .assert().success().stdout(contains("AB"));
}

#[test]
fn test_url_encode_cli() {
    cmd().args(["url-encode", "--input", "hello world"])
        .assert().success().stdout(contains("hello%20world"));
}

#[test]
fn test_url_decode_cli() {
    cmd().args(["url-decode", "--input", "hello%20world"])
        .assert().success().stdout(contains("hello world"));
}

#[test]
fn test_rot13_cli() {
    cmd().args(["rot13", "--input", "Hello"])
        .assert().success().stdout(contains("Uryyb"));
}

#[test]
fn test_rot13_roundtrip_cli() {
    let encoded = cmd().args(["rot13", "--input", "Secret Message"])
        .output().unwrap();
    let encoded_str = String::from_utf8(encoded.stdout).unwrap();
    cmd().args(["rot13", "--input", encoded_str.trim()])
        .assert().success().stdout(contains("Secret Message"));
}

#[test]
fn test_binary_encode_cli() {
    cmd().args(["binary-encode", "--input", "A"])
        .assert().success().stdout(contains("01000001"));
}

#[test]
fn test_binary_decode_cli() {
    cmd().args(["binary-decode", "--input", "01000001"])
        .assert().success().stdout(contains("A"));
}

#[test]
fn test_reverse_cli() {
    cmd().args(["reverse", "--input", "hello"])
        .assert().success().stdout(contains("olleh"));
}

#[test]
fn test_to_upper_cli() {
    cmd().args(["to-upper", "--input", "hello"])
        .assert().success().stdout(contains("HELLO"));
}

#[test]
fn test_to_lower_cli() {
    cmd().args(["to-lower", "--input", "HELLO"])
        .assert().success().stdout(contains("hello"));
}

// ── Hashing ───────────────────────────────────────────────────────────────
#[test]
fn test_md5_cli() {
    cmd().args(["md5", "--input", "hello"])
        .assert().success().stdout(contains("5d41402abc4b2a76b9719d911017c592"));
}

#[test]
fn test_sha1_cli() {
    cmd().args(["sha1", "--input", "hello"])
        .assert().success().stdout(contains("aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"));
}

#[test]
fn test_sha256_cli() {
    cmd().args(["sha256", "--input", "hello"])
        .assert().success().stdout(contains("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"));
}

// ── Crypto ────────────────────────────────────────────────────────────────
#[test]
fn test_xor_roundtrip_cli() {
    let encoded = cmd().args(["xor", "--input", "Hello", "--key", "K"])
        .output().unwrap();
    let hex = String::from_utf8(encoded.stdout).unwrap();
    cmd().args(["xor-decode", "--input", hex.trim(), "--key", "K"])
        .assert().success().stdout(contains("Hello"));
}

// ── Extraction ────────────────────────────────────────────────────────────
#[test]
fn test_extract_ip_cli() {
    cmd().args(["extract-ip", "--input", "Server at 192.168.1.1 and 10.0.0.1"])
        .assert().success()
        .stdout(contains("192.168.1.1"))
        .stdout(contains("10.0.0.1"));
}

#[test]
fn test_extract_url_cli() {
    cmd().args(["extract-url", "--input", "See https://example.com for details"])
        .assert().success().stdout(contains("https://example.com"));
}

#[test]
fn test_extract_email_cli() {
    cmd().args(["extract-email", "--input", "Contact admin@example.com now"])
        .assert().success().stdout(contains("admin@example.com"));
}

// ── Analysis ──────────────────────────────────────────────────────────────
#[test]
fn test_stats_cli() {
    cmd().args(["stats", "--input", "Hello World"])
        .assert().success()
        .stdout(contains("Words"))
        .stdout(contains("Characters"));
}

#[test]
fn test_entropy_cli() {
    cmd().args(["entropy", "--input", "hello world"])
        .assert().success().stdout(contains("bits/byte"));
}

#[test]
fn test_entropy_uniform_cli() {
    cmd().args(["entropy", "--input", "aaaaaaaaaa"])
        .assert().success().stdout(contains("0.0000"));
}

#[test]
fn test_char_freq_cli() {
    cmd().args(["char-freq", "--input", "aabbc", "--top", "3"])
        .assert().success().stdout(contains("Count"));
}

// ── Error handling ────────────────────────────────────────────────────────
#[test]
fn test_base64_decode_invalid() {
    cmd().args(["base64-decode", "--input", "not-valid-base64!!!"])
        .assert().failure();
}

#[test]
fn test_hex_decode_invalid() {
    cmd().args(["hex-decode", "--input", "xyz"])
        .assert().failure();
}

#[test]
fn test_unknown_subcommand() {
    cmd().args(["nonexistent-operation"])
        .assert().failure();
}
