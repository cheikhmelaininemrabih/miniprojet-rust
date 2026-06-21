use std::collections::HashMap;

pub struct TextStats {
    pub chars: usize,
    pub words: usize,
    pub lines: usize,
    pub unique_chars: usize,
    pub most_common: Option<(char, usize)>,
}

pub fn text_stats(input: &str) -> TextStats {
    let chars = input.chars().count();
    let words = input.split_whitespace().count();
    let lines = if input.is_empty() { 0 } else { input.lines().count() };

    let mut freq: HashMap<char, usize> = HashMap::new();
    for c in input.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }

    let unique_chars = freq.len();
    let most_common = freq.into_iter().max_by_key(|&(_, count)| count);

    TextStats { chars, words, lines, unique_chars, most_common }
}

/// Shannon entropy in bits per byte
pub fn entropy(input: &str) -> f64 {
    if input.is_empty() {
        return 0.0;
    }
    let bytes = input.as_bytes();
    let len = bytes.len() as f64;
    let mut freq = [0u64; 256];
    for &b in bytes {
        freq[b as usize] += 1;
    }
    freq.iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f64 / len;
            -p * p.log2()
        })
        .sum()
}

pub fn char_frequency(input: &str) -> Vec<(char, usize, f64)> {
    let mut freq: HashMap<char, usize> = HashMap::new();
    for c in input.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }
    let total = input.chars().count() as f64;
    let mut result: Vec<(char, usize, f64)> = freq
        .into_iter()
        .map(|(c, n)| (c, n, n as f64 / total * 100.0))
        .collect();
    result.sort_by_key(|b| std::cmp::Reverse(b.1));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_empty() {
        assert_eq!(entropy(""), 0.0);
    }

    #[test]
    fn test_entropy_uniform() {
        // Single repeated character -> entropy = 0
        assert_eq!(entropy("aaaa"), 0.0);
    }

    #[test]
    fn test_entropy_two_chars() {
        // "ab" repeated -> entropy = 1.0
        let e = entropy("abababab");
        assert!((e - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_entropy_range() {
        let e = entropy("Hello, World! This is a test string for entropy.");
        assert!(e > 0.0 && e <= 8.0);
    }

    #[test]
    fn test_text_stats() {
        let s = "Hello World\nFoo Bar";
        let stats = text_stats(s);
        assert_eq!(stats.words, 4);
        assert_eq!(stats.lines, 2);
        assert!(stats.chars > 0);
    }

    #[test]
    fn test_char_frequency() {
        let freq = char_frequency("aab");
        assert_eq!(freq[0].0, 'a');
        assert_eq!(freq[0].1, 2);
    }
}
