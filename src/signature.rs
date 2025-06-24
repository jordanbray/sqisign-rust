use crate::{Level, PublicKey};

/// Store _just_ the `Signature` portion of the signed message
/// of lvl1 SQISign.
///
/// This does not store a copy of the message.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Signature<L: Level> {
    pub(crate) bytes: L::Signature,
}

impl<L: Level> Signature<L> {
    /// Get the bytes of the signature.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes.as_ref()
    }

    /// Create a signature from a set of bytes.
    pub fn from_bytes(bytes: L::Signature) -> Signature<L> {
        Signature { bytes }
    }

    /// Verify a signature is valid given a public key.
    pub fn verify(&self, msg: &[u8], public_key: &PublicKey<L>) -> bool {
        unsafe {
            L::verify(
                msg.as_ptr(),
                msg.len(),
                self.bytes.as_ref().as_ptr(),
                self.bytes.as_ref().len(),
                public_key.bytes().as_ref().as_ptr(),
            ) == 0
        }
    }
}
