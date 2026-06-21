/// XOR each byte of input with repeating key bytes. Output as hex.
pub fn xor(input: &str, key: &str) -> String {
    if key.is_empty() {
        return hex::encode(input.as_bytes());
    }
    let key_bytes = key.as_bytes();
    let result: Vec<u8> = input
        .bytes()
        .enumerate()
        .map(|(i, b)| b ^ key_bytes[i % key_bytes.len()])
        .collect();
    hex::encode(result)
}

/// XOR hex-encoded input with repeating key bytes. Output as UTF-8 string.
pub fn xor_decode(hex_input: &str, key: &str) -> Result<String, String> {
    let bytes = hex::decode(hex_input.trim().replace(' ', ""))
        .map_err(|e| format!("Hex decode error: {e}"))?;
    if key.is_empty() {
        return String::from_utf8(bytes).map_err(|e| format!("UTF-8 error: {e}"));
    }
    let key_bytes = key.as_bytes();
    let result: Vec<u8> = bytes
        .iter()
        .enumerate()
        .map(|(i, &b)| b ^ key_bytes[i % key_bytes.len()])
        .collect();
    String::from_utf8(result).map_err(|e| format!("UTF-8 decode error: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_roundtrip() {
        let original = "Hello, World!";
        let key = "secret";
        let encoded = xor(original, key);
        assert_eq!(xor_decode(&encoded, key).unwrap(), original);
    }

    #[test]
    fn test_xor_with_single_byte_key() {
        let original = "AAA";
        let key = "A";
        // 0x41 ^ 0x41 = 0x00
        assert_eq!(xor(original, key), "000000");
    }

    #[test]
    fn test_xor_empty_key() {
        let original = "AB";
        let encoded = xor(original, "");
        assert_eq!(encoded, "4142");
    }
}
