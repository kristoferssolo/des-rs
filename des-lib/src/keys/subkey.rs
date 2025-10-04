use bit_wrap::BitWrapper;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, BitWrapper)]
#[bit_width(48)]
pub struct Subkey(u64);
