use base64::{engine::general_purpose::STANDARD, Engine};

// ── Base64 ────────────────────────────────────────────────────────────────
pub fn base64_encode(input: &str) -> String {
    STANDARD.encode(input.as_bytes())
}

pub fn base64_decode(input: &str) -> Result<String, String> {
    let bytes = STANDARD
        .decode(input.trim())
        .map_err(|e| format!("Base64 decode error: {e}"))?;
    String::from_utf8(bytes).map_err(|e| format!("UTF-8 decode error: {e}"))
}

// ── Hex ───────────────────────────────────────────────────────────────────
pub fn hex_encode(input: &str) -> String {
    hex::encode(input.as_bytes())
}

pub fn hex_decode(input: &str) -> Result<String, String> {
    let bytes = hex::decode(input.trim().replace(' ', ""))
        .map_err(|e| format!("Hex decode error: {e}"))?;
    String::from_utf8(bytes).map_err(|e| format!("UTF-8 decode error: {e}"))
}

// ── URL encoding ──────────────────────────────────────────────────────────
pub fn url_encode(input: &str) -> String {
    urlencoding::encode(input).into_owned()
}

pub fn url_decode(input: &str) -> Result<String, String> {
    urlencoding::decode(input)
        .map(|s| s.into_owned())
        .map_err(|e| format!("URL decode error: {e}"))
}

// ── ROT13 ─────────────────────────────────────────────────────────────────
pub fn rot13(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            'a'..='m' | 'A'..='M' => (c as u8 + 13) as char,
            'n'..='z' | 'N'..='Z' => (c as u8 - 13) as char,
            _ => c,
        })
        .collect()
}

// ── Binary ────────────────────────────────────────────────────────────────
pub fn binary_encode(input: &str) -> String {
    input
        .bytes()
        .map(|b| format!("{:08b}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn binary_decode(input: &str) -> Result<String, String> {
    let bytes: Result<Vec<u8>, _> = input
        .split_whitespace()
        .map(|s| u8::from_str_radix(s, 2).map_err(|e| format!("Binary decode error: {e}")))
        .collect();
    let bytes = bytes?;
    String::from_utf8(bytes).map_err(|e| format!("UTF-8 decode error: {e}"))
}

// ── Reverse ───────────────────────────────────────────────────────────────
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

// ── Case ─────────────────────────────────────────────────────────────────
pub fn to_upper(input: &str) -> String {
    input.to_uppercase()
}

pub fn to_lower(input: &str) -> String {
    input.to_lowercase()
}

// ── Tests ─────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_roundtrip() {
        let original = "Hello, RustChef!";
        assert_eq!(base64_decode(&base64_encode(original)).unwrap(), original);
    }

    #[test]
    fn test_base64_encode() {
        assert_eq!(base64_encode("hello"), "aGVsbG8=");
    }

    #[test]
    fn test_base64_decode() {
        assert_eq!(base64_decode("aGVsbG8=").unwrap(), "hello");
    }

    #[test]
    fn test_hex_roundtrip() {
        let original = "deadbeef";
        assert_eq!(hex_decode(&hex_encode(original)).unwrap(), original);
    }

    #[test]
    fn test_hex_encode() {
        assert_eq!(hex_encode("AB"), "4142");
    }

    #[test]
    fn test_url_roundtrip() {
        let original = "hello world & more=stuff";
        assert_eq!(url_decode(&url_encode(original)).unwrap(), original);
    }

    #[test]
    fn test_url_encode() {
        assert_eq!(url_encode("hello world"), "hello%20world");
    }

    #[test]
    fn test_rot13_roundtrip() {
        let original = "Hello World";
        assert_eq!(rot13(&rot13(original)), original);
    }

    #[test]
    fn test_rot13() {
        assert_eq!(rot13("abc"), "nop");
        assert_eq!(rot13("xyz"), "klm");
    }

    #[test]
    fn test_binary_roundtrip() {
        let original = "Hi!";
        assert_eq!(binary_decode(&binary_encode(original)).unwrap(), original);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("hello"), "olleh");
        assert_eq!(reverse("abcde"), "edcba");
    }

    #[test]
    fn test_case() {
        assert_eq!(to_upper("hello"), "HELLO");
        assert_eq!(to_lower("WORLD"), "world");
    }
}
