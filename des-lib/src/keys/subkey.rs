use crate::keys::key::Key;
use bit_wrap::BitWrapper;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, BitWrapper)]
#[bit_width(48)]
pub struct Subkey(u64);

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Subkeys([Subkey; 16]);

impl From<Key> for Subkey {
    fn from(_key: Key) -> Self {
        todo!("when other functions are moved to type methods, imlmenet this");
    }
}

impl TryFrom<[u64; 16]> for Subkeys {
    type Error = SubkeyError;
    fn try_from(keys: [u64; 16]) -> Result<Self, Self::Error> {
        let mut subkeys = [Subkey::default(); 16];

        for (idx, &key) in keys.iter().enumerate() {
            let subkey = Subkey::try_from(key)?;
            subkeys[idx] = subkey;
        }

        Ok(Subkeys(subkeys))
    }
}

impl From<Subkeys> for [Subkey; 16] {
    fn from(subkeys: Subkeys) -> Self {
        subkeys.0
    }
}

impl AsRef<[Subkey; 16]> for Subkeys {
    fn as_ref(&self) -> &[Subkey; 16] {
        &self.0
    }
}

impl PartialEq<u64> for Subkey {
    fn eq(&self, other: &u64) -> bool {
        &self.0 == other
    }
}
