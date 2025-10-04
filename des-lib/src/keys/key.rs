use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Key(u64);

impl Key {
    // #[macro_use]
    // pub fn new(key: u64) -> Self {
    //     key.into()
    // }
}

impl From<u64> for Key {
    fn from(key: u64) -> Self {
        Self(key)
    }
}

impl Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key(0x{:016X})", self.0)
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:016X}", self.0)
    }
}
