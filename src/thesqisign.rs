unsafe extern "C" {
    /// SQISign impl to generate their own keys
    pub(crate) fn sqisign_lvl1_broadwell_sqisign_keypair(
        public_key: *mut u8,
        private_key: *mut u8,
    ) -> i32;

    /// Sign a message.  Note their implementation copies `message`
    /// into `signed_message`.  I have attempted to make that clear
    /// with variable names.
    ///
    /// `signed_message` must have a length equal to `message_len + SIGNATURE_BYTES`.
    pub(crate) fn sqisign_lvl1_broadwell_sqisign_sign(
        signed_message: *mut u8,
        signed_message_len: *mut usize,
        message: *const u8,
        message_len: usize,
        private_key: *const u8,
    ) -> i32;

    /// Verify a message was signed correctly.
    pub(crate) fn sqisign_lvl1_broadwell_sqisign_verify(
        message: *const u8,
        message_len: usize,
        signed_message: *const u8,
        signed_message_len: usize,
        public_key: *const u8,
    ) -> i32;

    /// SQISign impl to generate their own keys
    pub(crate) fn sqisign_lvl3_broadwell_sqisign_keypair(
        public_key: *mut u8,
        private_key: *mut u8,
    ) -> i32;

    /// Sign a message.  Note their implementation copies `message`
    /// into `signed_message`.  I have attempted to make that clear
    /// with variable names.
    ///
    /// `signed_message` must have a length equal to `message_len + SIGNATURE_BYTES`.
    pub(crate) fn sqisign_lvl3_broadwell_sqisign_sign(
        signed_message: *mut u8,
        signed_message_len: *mut usize,
        message: *const u8,
        message_len: usize,
        private_key: *const u8,
    ) -> i32;

    /// Verify a message was signed correctly.
    pub(crate) fn sqisign_lvl3_broadwell_sqisign_verify(
        message: *const u8,
        message_len: usize,
        signed_message: *const u8,
        signed_message_len: usize,
        public_key: *const u8,
    ) -> i32;

    /// SQISign impl to generate their own keys
    pub(crate) fn sqisign_lvl5_broadwell_sqisign_keypair(
        public_key: *mut u8,
        private_key: *mut u8,
    ) -> i32;

    /// Sign a message.  Note their implementation copies `message`
    /// into `signed_message`.  I have attempted to make that clear
    /// with variable names.
    ///
    /// `signed_message` must have a length equal to `message_len + SIGNATURE_BYTES`.
    pub(crate) fn sqisign_lvl5_broadwell_sqisign_sign(
        signed_message: *mut u8,
        signed_message_len: *mut usize,
        message: *const u8,
        message_len: usize,
        private_key: *const u8,
    ) -> i32;

    /// Verify a message was signed correctly.
    pub(crate) fn sqisign_lvl5_broadwell_sqisign_verify(
        message: *const u8,
        message_len: usize,
        signed_message: *const u8,
        signed_message_len: usize,
        public_key: *const u8,
    ) -> i32;

    /// Initialize the RNG used for creating a seed.
    pub(crate) fn randombytes_init(seed: *const u8, moreseed: *const u8, _: i32) -> i32;
}
