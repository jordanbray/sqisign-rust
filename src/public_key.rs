use crate::Level;

/// This stores a sqisign lvl1 public key.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct PublicKey<L: Level> {
    pub(crate) bytes: L::PublicKey,
}

impl<L: Level> PublicKey<L> {
    /// Get the bytes of the public key.  That's actually all you
    /// are allowed to do with `PublicKey`s.
    pub fn bytes(&self) -> &[u8] {
        self.bytes.as_ref()
    }

    /// Create a public key from bytes.
    pub fn from_bytes(bytes: L::PublicKey) -> PublicKey<L> {
        PublicKey { bytes }
    }
}
