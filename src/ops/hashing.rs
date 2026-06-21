use digest::Digest;
use md5::Md5;
use sha1::Sha1;
use sha2::Sha256;

pub fn md5(input: &str) -> String {
    let mut h = Md5::new();
    h.update(input.as_bytes());
    hex::encode(h.finalize())
}

pub fn sha1(input: &str) -> String {
    let mut h = Sha1::new();
    h.update(input.as_bytes());
    hex::encode(h.finalize())
}

pub fn sha256(input: &str) -> String {
    let mut h = Sha256::new();
    h.update(input.as_bytes());
    hex::encode(h.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        assert_eq!(md5("hello"), "5d41402abc4b2a76b9719d911017c592");
    }

    #[test]
    fn test_sha1() {
        assert_eq!(sha1("hello"), "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
    }

    #[test]
    fn test_sha256() {
        assert_eq!(
            sha256("hello"),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_md5_empty() {
        assert_eq!(md5(""), "d41d8cd98f00b204e9800998ecf8427e");
    }
}
