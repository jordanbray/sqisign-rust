/// This trait allows low-level access to the SQISign library.  This
/// trait is implemented by `Lvl1`, `Lvl3`, and `Lvl5`, and is
/// used by the `PublicKey`, `PrivateKey`, and `Signature` structs.
///
/// This trait is private to disallow users from calling the
/// unsafe functions.
pub(crate) trait InternalLevel {
    /// Generate a keypair using the appropriate level function.
    unsafe fn keypair(public_key: *mut u8, private_key: *mut u8) -> i32;

    /// Sign a message using the appropriate level function.
    unsafe fn sign(
        signed_message: *mut u8,
        signed_message_len: *mut usize,
        message: *const u8,
        message_len: usize,
        private_key: *const u8,
    ) -> i32;

    /// Verify a message using the appropriate level function.
    unsafe fn verify(
        message: *const u8,
        message_len: usize,
        signed_message: *const u8,
        signed_message_len: usize,
        public_key: *const u8,
    ) -> i32;
}
