use thiserror::Error;

use crate::keys::{key::KeyError, subkey::SubkeyError};

#[derive(Debug, Error)]
pub enum DesError {
    /// Key value exceeds the maximum allowed value for the bit width
    #[error("Key value {value} exceeds maximum {max} for {width}-bit type")]
    KeyOutOfRange {
        value: u64, // Raw value that was too large
        max: u64,   // Maximum allowed value (2^bit_width - 1)
        width: u8,  // Bit width of the key type
    },

    /// Failed to parse a hex or binary string representation
    #[error("Failed to parse key string: {0}")]
    ParseError(#[from] std::num::ParseIntError),

    /// Failed to parse from a string with invalid format
    #[error("Invalid key format: {0}")]
    InvalidFormat(String),

    /// Bitfield operation with invalid range (high < low)
    #[error("Invalid bitfield range: low={low}, high={high} (must have low <= high)")]
    InvalidBitfieldRange { low: u8, high: u8 },

    /// Attempted to set a bit beyond the valid bit width
    #[error("Bit index {bit} out of range for {width}-bit type")]
    InvalidBitIndex { bit: u8, width: u8 },

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl DesError {
    pub fn unknown(input: impl Into<String>) -> Self {
        Self::Unknown(input.into())
    }
}

macro_rules! impl_from_key_error_for_des {
    ($error_type:ty) => {
        impl From<$error_type> for DesError {
            fn from(error: $error_type) -> Self {
                type Input = $error_type;
                match error {
                    Input::ValueOutOfRange { value, max, width } => Self::KeyOutOfRange {
                        value: value as u64,
                        max: max as u64,
                        width,
                    },
                    Input::ParseError(err) => Self::ParseError(err),
                    Input::Unknown(msg) => Self::Unknown(msg),
                }
            }
        }
    };
}

impl_from_key_error_for_des!(SubkeyError);
impl_from_key_error_for_des!(KeyError);
