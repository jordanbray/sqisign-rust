/// This is a completely opaque error type.  No information is provided.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Error;

pub type Result<T> = std::result::Result<T, Error>;
