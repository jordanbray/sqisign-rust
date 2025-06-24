use crate::internal_level::InternalLevel;

/// This is the public Level trait, which contains values someone could actually use
/// safely.
#[allow(private_bounds)]
pub trait Level: InternalLevel {
    /// How many bytes are in a public key?
    const PUBLIC_KEY_BYTES: usize;

    /// How many bytes are in a private key?
    const PRIVATE_KEY_BYTES: usize;

    /// How many bytes are in a signature?
    const SIGNATURE_BYTES: usize;

    // Developers note: The following 6 (!) items can be removed in
    // preference for [u8; *_BYTES], but require unstable rust features
    // to do so.  To keep the implementation on stable, I simply
    // redefine each of these types and values for each level,
    // and use those in the implementations.

    /// This _should_ be removed in preference of [u8; Self::PUBLIC_KEY_BYTES],
    /// but can't on stable rust at the moment.
    type PublicKey: AsRef<[u8]>
        + AsMut<[u8]>
        + Copy
        + Clone
        + PartialEq
        + PartialOrd
        + std::fmt::Debug;

    /// This _should_ be removed in preference of [u8; Self::PRIVATE_KEY_BYTES],
    /// but can't on stable rust at the moment.
    type PrivateKey: AsRef<[u8]>
        + AsMut<[u8]>
        + Copy
        + Clone
        + PartialEq
        + PartialOrd
        + std::fmt::Debug;

    /// This _should_ be removed in preference of [u8; Self::SIGNATURE_KEY_BYTES],
    /// but can't on stable rust at the moment.
    type Signature: AsRef<[u8]>
        + AsMut<[u8]>
        + Copy
        + Clone
        + PartialEq
        + PartialOrd
        + std::fmt::Debug;

    /// This _should_ be removed in preference of [0; Self::PUBLIC_KEY_BYTES],
    /// but can't on stable rust at the moment.
    const EMPTY_PUBLIC_KEY: Self::PublicKey;

    /// This _should_ be removed in preference of [0; Self::PRIVATE_KEY_BYTES],
    /// but can't on stable rust at the moment.
    const EMPTY_PRIVATE_KEY: Self::PrivateKey;

    /// This _should_ be removed in preference of [0; Self::SIGNATURE_KEY_BYTES],
    /// but can't on stable rust at the moment.
    const EMPTY_SIGNATURE: Self::Signature;
}
