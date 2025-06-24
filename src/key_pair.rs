use crate::{Error, Level, PrivateKey, PublicKey, Result};

use crate::thesqisign::randombytes_init;

/// This stores a SQISign `KeyPair`, which is both a public key and a
/// private key.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct KeyPair<L: Level> {
    private_key: PrivateKey<L>,
    public_key: PublicKey<L>,
}

impl<L: Level> KeyPair<L> {
    /// Get the private key portion of the `KeyPair`.
    pub fn private_key(&self) -> &PrivateKey<L> {
        &self.private_key
    }

    /// Get the public key portion of the `KeyPair`.
    pub fn public_key(&self) -> &PublicKey<L> {
        &self.public_key
    }

    /// Create a new `KeyPair` using the SQISign official library.
    pub fn new(seed: [u8; 48]) -> Result<KeyPair<L>> {
        let res = unsafe { randombytes_init(seed.as_ptr(), std::ptr::null(), 256) };
        if res != 0 {
            return Err(Error);
        }

        let mut private_key = PrivateKey::<L> {
            bytes: L::EMPTY_PRIVATE_KEY,
        };
        let mut public_key = PublicKey::<L> {
            bytes: L::EMPTY_PUBLIC_KEY,
        };

        let result = {
            stacker::maybe_grow(8 << 20, 8 << 20, || unsafe {
                L::keypair(
                    public_key.bytes.as_mut().as_mut_ptr(),
                    private_key.bytes.as_mut().as_mut_ptr(),
                )
            })
        };
        if result != 0 {
            Err(Error)
        } else {
            Ok(KeyPair {
                private_key,
                public_key,
            })
        }
    }
}
