//! This is a rust-ideomatic wrapper for the SQISign library.
//!
//! Only stuff the user should touch is public, and the unsafe C calls
//! are all wrapped by structures which prevent misuse.
//!
//! Example:
//! ```
//!     use rand::prelude::*;
//!     use sqisign_rust::*;
//!     
//!     fn main() -> Result<()> {
//!         let mut rng = rand::thread_rng();
//!         let mut seed = [0; 48];
//!         rng.fill(&mut seed);
//!         let key_pair = KeyPair::<Lvl3>::new(seed)?;
//!         let msg_a = b"Hello, World!!!";
//!         let msg_b = b"Goodbye, World!!!";
//!         let sig = key_pair.private_key().sign(msg_a)?;
//!         assert!(sig.verify(msg_a, key_pair.public_key()));
//!         assert!(!sig.verify(msg_b, key_pair.public_key()));
//!         Ok(())
//!     }
//! ```

mod error;
mod internal_level;
mod key_pair;
mod level;
mod lvl1;
mod lvl3;
mod lvl5;
mod private_key;
mod public_key;
mod signature;
mod thesqisign;

#[cfg(test)]
mod tests;

pub use error::{Error, Result};
pub use key_pair::KeyPair;
pub use level::Level;
pub use lvl1::Lvl1;
pub use lvl3::Lvl3;
pub use lvl5::Lvl5;
pub use private_key::PrivateKey;
pub use public_key::PublicKey;
pub use signature::Signature;
