use clap::{Parser, Subcommand, ValueEnum};
use std::{
    fmt::{Display, LowerHex, UpperHex},
    fs::read_to_string,
    num::IntErrorKind,
    path::PathBuf,
    str::FromStr,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValueError {
    #[error("String contains no content")]
    EmptyString,

    #[error("File '{0}' contains no content")]
    EmptyFile(PathBuf),

    #[error("Failed to find file '{0}'. File does not exist")]
    MissingFile(PathBuf),

    #[error("Failed to read file '{0}'. Cannot read file contents")]
    FileReadingError(PathBuf),

    #[error("Invalid number format: {0}")]
    InvalidFormat(String),

    #[error("Invalid byte string: must be exactly 8 ASCII characters")]
    InvalidByteString,

    #[error("String-to-u64 conversion error: {0}")]
    ConversionError(String),
}

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub operation: Operation,

    /// Key used to encrypt/decrypt data (64-bit number, string, or path to file)
    #[arg(short = 'k', long, value_parser = Value::from_str, required = true)]
    pub key: Value,

    /// The text to encrypt/decrypt data (64-bit number, string, or path to file)
    #[arg(value_name = "TEXT", value_parser = Value::from_str, required = true)]
    pub text: Value,
}

#[derive(Debug, Clone, Subcommand, Default)]
pub enum Operation {
    /// Encrypt data
    #[default]
    Encrypt,
    /// Decrypt data
    Decrypt {
        /// Output format for decrypted data
        #[arg(short = 'f', long, value_enum)]
        output_format: Option<OutputFormat>,
    },
}

#[derive(Debug, Clone, Default, ValueEnum)]
pub enum OutputFormat {
    /// Binary output
    Binary,
    /// Octal output (fixed typo)
    Octal,
    /// Decimal output
    Decimal,
    /// Hexadecimal output
    #[default]
    Hex,
    /// Text output (ASCII)
    Text,
}

#[derive(Debug, Clone, Copy)]
pub struct Value(u64);

impl Value {
    #[inline]
    #[must_use]
    pub const fn as_64(self) -> u64 {
        self.0
    }
}

impl From<Value> for u64 {
    fn from(value: Value) -> Self {
        value.as_64()
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl FromStr for Value {
    type Err = ValueError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse::<u64>() {
            return Ok(Self(num));
        }

        let path = PathBuf::from(s);
        if path.exists() && path.is_file() {
            if let Ok(contents) = read_to_string(&path) {
                let value = parse_string_to_u64(&contents)?;
                return Ok(Self(value));
            }
            return Err(ValueError::FileReadingError(path));
        }

        let value = parse_string_to_u64(&s)?;
        Ok(Self(value))
    }
}

fn parse_string_to_u64(s: &str) -> Result<u64, ValueError> {
    let trimmed = s.trim();

    if trimmed.is_empty() {
        return Err(ValueError::EmptyString);
    }

    // Hexadecimal with 0x/0X prefix
    if trimmed.starts_with("0x") || trimmed.starts_with("0X") {
        let hex_str = &trimmed[2..].trim_start_matches('0');
        if hex_str.is_empty() {
            return Ok(0); // 0x000 ->
        }
        return u64::from_str_radix(hex_str, 16)
            .map_err(|e| ValueError::InvalidFormat(format!("Hex parsing failed: {e}")));
    }

    // Binary with 0b/0B prefix
    if trimmed.starts_with("0b") || trimmed.starts_with("0B") {
        let bin_str = &trimmed[2..].trim_start_matches('0');
        if bin_str.is_empty() {
            return Ok(0); // 0b000 -> 0
        }
        if !bin_str.chars().all(|ch| ch == '0' || ch == '1') {
            return Err(ValueError::InvalidFormat(
                "Binary string contains invalid characters".into(),
            ));
        }
        return u64::from_str_radix(bin_str, 2)
            .map_err(|e| ValueError::InvalidFormat(format!("Binary parsing failed: {e}")));
    }

    // 8-character ASCII string conversion to u64
    if trimmed.len() == 8 {
        return ascii_string_to_u64(trimmed);
    }

    // Regular decimal parsing
    trimmed.parse::<u64>().map_err(|e| {
        ValueError::InvalidFormat(match e.kind() {
            IntErrorKind::InvalidDigit => "contains invalid digits".into(),
            IntErrorKind::PosOverflow => "number too large for u64".into(),
            IntErrorKind::NegOverflow => "negative numbers not allowed".into(),
            IntErrorKind::Empty => "empty string".into(),
            IntErrorKind::Zero => "invalid zero".into(),
            _ => format!("parsing error: {e}"),
        })
    })
}

fn ascii_string_to_u64(s: &str) -> Result<u64, ValueError> {
    if s.len() != 8 {
        return Err(ValueError::InvalidByteString);
    }

    // Ensure all characters are valid ASCII (0-127)
    if !s.bytes().all(|b| b <= 127) {
        return Err(ValueError::ConversionError(
            "String contains non-ASCII characters".into(),
        ));
    }

    let mut bytes = [0; 8];
    for (idx, byte) in s.bytes().enumerate() {
        bytes[idx] = byte;
    }

    Ok(u64::from_le_bytes(bytes))
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0b}", self.0)
    }
}

impl UpperHex for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016X}", self.0)
    }
}

impl LowerHex for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016x}", self.0)
    }
}
