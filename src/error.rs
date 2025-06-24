/// This is a completely opaque error type.  No information is provided.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Error;

/// This is a `Result` type, which is returned by the crate on any fallible
/// function.
pub type Result<T> = std::result::Result<T, Error>;
