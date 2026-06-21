mod ops;

use clap::{Parser, Subcommand};
use std::io::{self, Read};

use ops::{analysis, crypto, encoding, extraction, hashing};

/// RustChef — A CyberChef-inspired CLI data transformation tool
/// Author: Cheikh Melainine (22027) — SUPNUM Master Cybersécurité
#[derive(Parser)]
#[command(
    name = "rustchef",
    version = "1.0.0",
    author = "Cheikh Melainine <22027@supnum.mr>",
    about = "CyberChef-inspired CLI: encode, decode, hash, extract, analyse",
    long_about = "RustChef provides 20+ data transformation operations from the terminal.\nInput is read from --input flag or stdin if not provided.\n\nExample:\n  echo 'hello' | rustchef base64-encode\n  rustchef sha256 --input 'hello'\n  rustchef extract-ip --input 'Server: 192.168.1.1'"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode input to Base64
    #[command(name = "base64-encode", alias = "b64e")]
    Base64Encode {
        /// Input string (reads from stdin if omitted)
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Decode Base64 input
    #[command(name = "base64-decode", alias = "b64d")]
    Base64Decode {
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Encode input to hexadecimal
    #[command(name = "hex-encode", alias = "hexe")]
    HexEncode {
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Decode hexadecimal input to string
    #[command(name = "hex-decode", alias = "hexd")]
    HexDecode {
        #[arg(short, long)]
        input: Option<String>,
    },

    /// URL-encode input (percent-encoding)
    #[command(name = "url-encode", alias = "urle")]
    UrlEncode {
        #[arg(short, long)]
        input: Option<String>,
    },

    /// URL-decode (percent-decode) input
    #[command(name = "url-decode", alias = "urld")]
    UrlDecode {
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Apply ROT13 substitution cipher
    #[command(name = "rot13")]
    Rot13 {
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Encode input to binary (space-separated 8-bit groups)
    #[command(name = "binary-encode", alias = "bine")]
    BinaryEncode {
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Decode binary (space-separated 8-bit groups) to string
    #[command(name = "binary-decode", alias = "bind")]
    BinaryDecode {
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Reverse the input string
    #[command(name = "reverse", alias = "rev")]
    Reverse {
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Convert input to UPPERCASE
    #[command(name = "to-upper", alias = "upper")]
    ToUpper {
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Convert input to lowercase
    #[command(name = "to-lower", alias = "lower")]
    ToLower {
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Compute MD5 hash of input
    #[command(name = "md5")]
    Md5 {
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Compute SHA-1 hash of input
    #[command(name = "sha1")]
    Sha1 {
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Compute SHA-256 hash of input
    #[command(name = "sha256")]
    Sha256 {
        #[arg(short, long)]
        input: Option<String>,
    },

    /// XOR input with a key (output: hex-encoded)
    #[command(name = "xor")]
    Xor {
        #[arg(short, long)]
        input: Option<String>,
        /// XOR key (repeating)
        #[arg(short, long, default_value = "")]
        key: String,
    },

    /// XOR-decode hex input with a key (output: plaintext)
    #[command(name = "xor-decode", alias = "xord")]
    XorDecode {
        #[arg(short, long)]
        input: Option<String>,
        #[arg(short, long, default_value = "")]
        key: String,
    },

    /// Extract all IPv4 addresses from input
    #[command(name = "extract-ip", alias = "ip")]
    ExtractIp {
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Extract all URLs from input
    #[command(name = "extract-url", alias = "url")]
    ExtractUrl {
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Extract all email addresses from input
    #[command(name = "extract-email", alias = "email")]
    ExtractEmail {
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Extract all domain names from input
    #[command(name = "extract-domain", alias = "domain")]
    ExtractDomain {
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Show text statistics (chars, words, lines, unique chars)
    #[command(name = "stats")]
    Stats {
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Calculate Shannon entropy of input (bits per byte)
    #[command(name = "entropy")]
    Entropy {
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Show character frequency table
    #[command(name = "char-freq", alias = "freq")]
    CharFreq {
        #[arg(short, long)]
        input: Option<String>,
        /// Limit to top N characters
        #[arg(short, long, default_value = "10")]
        top: usize,
    },
}

fn get_input(opt: Option<String>) -> Result<String, String> {
    match opt {
        Some(s) => Ok(s),
        None => {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .map_err(|e| format!("Failed to read stdin: {e}"))?;
            Ok(buf.trim_end_matches('\n').to_string())
        }
    }
}

fn run() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Base64Encode { input } => {
            println!("{}", encoding::base64_encode(&get_input(input)?));
        }
        Commands::Base64Decode { input } => {
            println!("{}", encoding::base64_decode(&get_input(input)?)?);
        }
        Commands::HexEncode { input } => {
            println!("{}", encoding::hex_encode(&get_input(input)?));
        }
        Commands::HexDecode { input } => {
            println!("{}", encoding::hex_decode(&get_input(input)?)?);
        }
        Commands::UrlEncode { input } => {
            println!("{}", encoding::url_encode(&get_input(input)?));
        }
        Commands::UrlDecode { input } => {
            println!("{}", encoding::url_decode(&get_input(input)?)?);
        }
        Commands::Rot13 { input } => {
            println!("{}", encoding::rot13(&get_input(input)?));
        }
        Commands::BinaryEncode { input } => {
            println!("{}", encoding::binary_encode(&get_input(input)?));
        }
        Commands::BinaryDecode { input } => {
            println!("{}", encoding::binary_decode(&get_input(input)?)?);
        }
        Commands::Reverse { input } => {
            println!("{}", encoding::reverse(&get_input(input)?));
        }
        Commands::ToUpper { input } => {
            println!("{}", encoding::to_upper(&get_input(input)?));
        }
        Commands::ToLower { input } => {
            println!("{}", encoding::to_lower(&get_input(input)?));
        }
        Commands::Md5 { input } => {
            let data = get_input(input)?;
            println!("MD5({:?}) = {}", data, hashing::md5(&data));
        }
        Commands::Sha1 { input } => {
            let data = get_input(input)?;
            println!("SHA1({:?}) = {}", data, hashing::sha1(&data));
        }
        Commands::Sha256 { input } => {
            let data = get_input(input)?;
            println!("SHA256({:?}) = {}", data, hashing::sha256(&data));
        }
        Commands::Xor { input, key } => {
            println!("{}", crypto::xor(&get_input(input)?, &key));
        }
        Commands::XorDecode { input, key } => {
            println!("{}", crypto::xor_decode(&get_input(input)?, &key)?);
        }
        Commands::ExtractIp { input } => {
            let items = extraction::extract_ips(&get_input(input)?);
            if items.is_empty() {
                eprintln!("No IPv4 addresses found.");
            } else {
                for item in &items {
                    println!("{item}");
                }
                eprintln!("[{} match(es) found]", items.len());
            }
        }
        Commands::ExtractUrl { input } => {
            let items = extraction::extract_urls(&get_input(input)?);
            if items.is_empty() {
                eprintln!("No URLs found.");
            } else {
                for item in &items {
                    println!("{item}");
                }
                eprintln!("[{} match(es) found]", items.len());
            }
        }
        Commands::ExtractEmail { input } => {
            let items = extraction::extract_emails(&get_input(input)?);
            if items.is_empty() {
                eprintln!("No email addresses found.");
            } else {
                for item in &items {
                    println!("{item}");
                }
                eprintln!("[{} match(es) found]", items.len());
            }
        }
        Commands::ExtractDomain { input } => {
            let items = extraction::extract_domains(&get_input(input)?);
            if items.is_empty() {
                eprintln!("No domains found.");
            } else {
                for item in &items {
                    println!("{item}");
                }
                eprintln!("[{} match(es) found]", items.len());
            }
        }
        Commands::Stats { input } => {
            let data = get_input(input)?;
            let s = analysis::text_stats(&data);
            println!("Characters : {}", s.chars);
            println!("Words      : {}", s.words);
            println!("Lines      : {}", s.lines);
            println!("Unique chars: {}", s.unique_chars);
            if let Some((c, n)) = s.most_common {
                let display = if c == ' ' { "SPACE".to_string() } else { c.to_string() };
                println!("Most common: {:?} ({} times)", display, n);
            }
        }
        Commands::Entropy { input } => {
            let data = get_input(input)?;
            let e = analysis::entropy(&data);
            println!("Entropy: {:.4} bits/byte", e);
            let label = match e as u32 {
                0 => "Uniform (no randomness)",
                1..=3 => "Low entropy (structured/text)",
                4..=6 => "Medium entropy (mixed content)",
                _ => "High entropy (random/encrypted/compressed)",
            };
            println!("Assessment: {label}");
        }
        Commands::CharFreq { input, top } => {
            let data = get_input(input)?;
            let freq = analysis::char_frequency(&data);
            println!("{:<10} {:>8} {:>10}", "Char", "Count", "Frequency");
            println!("{}", "-".repeat(32));
            for (c, count, pct) in freq.iter().take(top) {
                let display = match c {
                    ' ' => "SPACE".to_string(),
                    '\n' => "NEWLINE".to_string(),
                    '\t' => "TAB".to_string(),
                    _ => c.to_string(),
                };
                println!("{:<10} {:>8} {:>9.2}%", display, count, pct);
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
