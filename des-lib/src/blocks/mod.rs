use bit_wrap::BitWrapper;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, BitWrapper)]
#[bit_width(64)]
pub struct Block(u64);
