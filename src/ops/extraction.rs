use regex::Regex;

pub fn extract_ips(input: &str) -> Vec<String> {
    let re = Regex::new(r"\b(?:(?:25[0-5]|2[0-4]\d|[01]?\d\d?)\.){3}(?:25[0-5]|2[0-4]\d|[01]?\d\d?)\b").unwrap();
    re.find_iter(input).map(|m| m.as_str().to_string()).collect()
}

pub fn extract_urls(input: &str) -> Vec<String> {
    let re = Regex::new(r#"https?://[^\s<>"'\)\]]+"#).unwrap();
    re.find_iter(input).map(|m| m.as_str().to_string()).collect()
}

pub fn extract_emails(input: &str) -> Vec<String> {
    let re = Regex::new(r"\b[A-Za-z0-9._%+\-]+@[A-Za-z0-9.\-]+\.[A-Za-z]{2,}\b").unwrap();
    re.find_iter(input).map(|m| m.as_str().to_string()).collect()
}

pub fn extract_domains(input: &str) -> Vec<String> {
    let re = Regex::new(r"\b(?:[a-zA-Z0-9](?:[a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}\b").unwrap();
    re.find_iter(input).map(|m| m.as_str().to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_ips() {
        let text = "Server at 192.168.1.1 and gateway 10.0.0.1, also 999.999.999.999 is invalid";
        let ips = extract_ips(text);
        assert!(ips.contains(&"192.168.1.1".to_string()));
        assert!(ips.contains(&"10.0.0.1".to_string()));
        assert!(!ips.contains(&"999.999.999.999".to_string()));
    }

    #[test]
    fn test_extract_urls() {
        let text = "Visit https://example.com and http://test.org/path?q=1 for more";
        let urls = extract_urls(text);
        assert_eq!(urls.len(), 2);
        assert!(urls.contains(&"https://example.com".to_string()));
        assert!(urls.contains(&"http://test.org/path?q=1".to_string()));
    }

    #[test]
    fn test_extract_emails() {
        let text = "Contact admin@example.com or support@test.org for help";
        let emails = extract_emails(text);
        assert_eq!(emails.len(), 2);
        assert!(emails.contains(&"admin@example.com".to_string()));
    }

    #[test]
    fn test_extract_empty() {
        assert!(extract_ips("no ips here").is_empty());
        assert!(extract_urls("no urls here").is_empty());
        assert!(extract_emails("no emails here").is_empty());
    }
}
