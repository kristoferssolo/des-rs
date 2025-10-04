use bit_wrap::BitWrapper;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, BitWrapper)]
#[bit_width(48)]
pub struct Subkey(u64);

// impl TryFrom<u64> for Subkey {
//     type Error = SubkeyError;
//     fn try_from(key: u64) -> Result<Self, Self::Error> {
//         if key > Self::MAX {
//             return Err(SubkeyError::ValueOutOfRange(key));
//         }
//         Ok(Self(key))
//     }
// }
//
// impl From<u32> for Subkey {
//     fn from(value: u32) -> Self {
//         Self(u64::from(value))
//     }
// }
//
// impl From<u16> for Subkey {
//     fn from(value: u16) -> Self {
//         Self(u64::from(value))
//     }
// }
//
// impl From<u8> for Subkey {
//     fn from(value: u8) -> Self {
//         Self(u64::from(value))
//     }
// }
