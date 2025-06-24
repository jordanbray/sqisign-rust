use crate::Level;
use crate::internal_level::InternalLevel;
use crate::thesqisign::{
    sqisign_lvl5_broadwell_sqisign_keypair, sqisign_lvl5_broadwell_sqisign_sign,
    sqisign_lvl5_broadwell_sqisign_verify,
};

/// Empty type, contains the SQISign lvl5 implementations.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Lvl5;

impl InternalLevel for Lvl5 {
    unsafe fn keypair(public_key: *mut u8, private_key: *mut u8) -> i32 {
        unsafe { sqisign_lvl5_broadwell_sqisign_keypair(public_key, private_key) }
    }
    unsafe fn sign(
        signed_message: *mut u8,
        signed_message_len: *mut usize,
        message: *const u8,
        message_len: usize,
        private_key: *const u8,
    ) -> i32 {
        unsafe {
            sqisign_lvl5_broadwell_sqisign_sign(
                signed_message,
                signed_message_len,
                message,
                message_len,
                private_key,
            )
        }
    }
    unsafe fn verify(
        message: *const u8,
        message_len: usize,
        signed_message: *const u8,
        signed_message_len: usize,
        public_key: *const u8,
    ) -> i32 {
        unsafe {
            sqisign_lvl5_broadwell_sqisign_verify(
                message,
                message_len,
                signed_message,
                signed_message_len,
                public_key,
            )
        }
    }
}

impl Level for Lvl5 {
    const PUBLIC_KEY_BYTES: usize = 129;
    const PRIVATE_KEY_BYTES: usize = 701;
    const SIGNATURE_BYTES: usize = 292;

    type PublicKey = [u8; Self::PUBLIC_KEY_BYTES];
    type PrivateKey = [u8; Self::PRIVATE_KEY_BYTES];
    type Signature = [u8; Self::SIGNATURE_BYTES];

    const EMPTY_PUBLIC_KEY: Self::PublicKey = [0; Self::PUBLIC_KEY_BYTES];
    const EMPTY_PRIVATE_KEY: Self::PrivateKey = [0; Self::PRIVATE_KEY_BYTES];
    const EMPTY_SIGNATURE: Self::Signature = [0; Self::SIGNATURE_BYTES];
}
