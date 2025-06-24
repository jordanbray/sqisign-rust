use crate::{Error, Level, Result, Signature};

/// This structure represents a private key.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct PrivateKey<L: Level> {
    pub(crate) bytes: L::PrivateKey,
}

impl<L: Level> PrivateKey<L> {
    /// Get the bytes out of the private key.  These are
    /// already encoded in the smallest form possible.
    pub fn bytes(&self) -> &[u8] {
        self.bytes.as_ref()
    }

    /// Create a private key from bytes.
    pub fn from_bytes(bytes: L::PrivateKey) -> PrivateKey<L> {
        PrivateKey { bytes }
    }

    /// This signs a message into _only_ a `Signature`.
    ///
    /// Despite the sqisign public API, we do _not_ include
    /// the message with the `Signature`.
    pub fn sign(&self, msg: &[u8]) -> Result<Signature<L>> {
        let mut sig_bytes = vec![0; msg.len() + L::SIGNATURE_BYTES];
        let mut sig_len = 0;
        let result = stacker::maybe_grow(8 << 20, 8 << 20, || unsafe {
            L::sign(
                sig_bytes.as_mut_ptr(),
                &mut sig_len,
                msg.as_ptr(),
                msg.len(),
                self.bytes.as_ref().as_ptr(),
            )
        });
        if result != 0 {
            Err(Error)
        } else {
            let mut bytes = L::EMPTY_SIGNATURE;
            for i in 0..L::SIGNATURE_BYTES {
                bytes.as_mut()[i] = sig_bytes[i];
            }
            Ok(Signature { bytes })
        }
    }
}
