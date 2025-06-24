# Rust SQISign Wrapper

This is an ideomatic Rust wrapper for the official SQISign library
found here https://github.com/SQISign/the-sqisign/.  This is a post-quantum
signature library, which allows a private key to sign a message, and
a public key to verify the same message.

## Warning: Reasonably modern x86_64 only

This library makes the (opinionated) choice to only build the `broadwell`
version of the official library.  This needs to be corrected at some point, as
this only supports some processors.

## Warning: not published on crates.io yet

This library has not been published on `crates.io` yet.  I intend to publish it
once I am more comfortable that it will build in esoteric configurations, such
as those using Windows (or WebAssembly, ARM, old x86, etc).

## Why?

There are many reasons to get post-quantum cryptography in the hands of users
sooner rather than later.  This algorithm caught my attention because the size
of the public key and signature are very small.

Having a rust version of the library increases accessibility to the new
library, and gets more eyes (and fingers) hacking on the new algorithms.

## Building

Building this (right now) is somewhat complicated.  That's because:

1. We need the changes made in PR 7 on the official repository.
2. We need the changes made in the `main` branch on the official repository
   as well.

To accomplish this, I recommend checking out this repository, then
performing the following steps.  The following code was carefully crafted
to fail (on purpose) if you somehow mess up the steps.

This will be unnecessary once PR 7 is merged.

```bash
cd the-sqisign
git fetch origin pull/7/head:mybranch
git checkout mybranch
git merge main
git commit -m "Merged."
cd ..
cargo test --release
```

Once you get your repo updated, you are good to go.

## Docs

I have not published the documentation anywhere.  To view it, check out
the repo and use `cargo doc --open` to view it in your web browser.

## Safety

I have done everything I can to make this library safe for consumption, by
hiding the details of the C calling convention behind an `InternalLevel` trait
which is not publicly exposed.  Using that, I can verify (using the type
checker), that the correct amount of space is always allocated before calling
the needed functions.

## The Stack Problem

Rust uses a 2MiB stack when creating a thread.  That's a problem for the
official library, which uses 8MiB of stack space.  To avoid this problem,
I use the `stacker` crate which increases the stack size to accomodate
the increased memory usage.  This is all done automatically, so you can
live in blissful ignorance that it has been performed.

Basically, just use the library like you would any other library, create
as many threads as you like, and it's fine.  Don't worry about it.  You don't
need to get clever with the rust `ThreadBuilder` or anything like that.

## Architecture

This library exposes 3 types: `Lvl1`, `Lvl3`, and `Lvl5`.  They are
used as markers to determine which functions in `the-sqisign` to call,
and how much memory to allocate for keys/signatures.  They also expose the
number of bytes each `Level` uses for each object type.  This library also
exposes the `Level` trait to allow code to be generic over the `Level`.

This library also exposes 4 types to the user which contains useful data:

* `KeyPair<Level>` - Generate a `PublicKey<Level>` and `PrivateKey<Level>` using
                     a 48 byte seed.
* `PublicKey<Level>` - This stores the public key.
* `PrivateKey<Level>` - This stores a private key, and contains the `sign` function.
* `Signature<Level>` - This stores a signature, and contains the `verify` function.

In addition, `PublicKey<Level>`, `PrivateKey<Level>` and `Signature<Level>` all contain
a `bytes` and `from_bytes` method which gives you their contents without any ceremony.

## Generic Arrays

Ideally, each `Level` exposes a size for the public key, private key, and
signature as an associated constant.  Then, that constant can be used to create
arrays of known sizes.  Unfortunately, that is not possible, due to this issue:
https://github.com/rust-lang/rust/issues/76560 .

As a result, I added associated types to `Level` (which are fully defined in
each of `Lvl1`, `Lvl3`, and `Lvl5`).  Ideally, this can be cleaned up once the
associated rust issue is resolved.  At that point, I will prefer
`[u8; Self::PUBLIC_KEY_BYTES]` and friends for clarity.

## Usage

The following code sample shows an example of generating a public/private key, and
verifying that signatures match (or don't match) given the data.

```rust
use rand::prelude::*;
use sqisign_rust::*;

fn main() -> Result<()> {
    let mut rng = rand::thread_rng();
    let mut seed = [0; 48];
    rng.fill(&mut seed);
    let key_pair = KeyPair::<Lvl3>::new(seed)?;
    let msg_a = b"Hello, World!!!";
    let msg_b = b"Goodbye, World!!!";
    let sig = key_pair.private_key().sign(msg_a)?;
    assert!(sig.verify(msg_a, key_pair.public_key()));
    assert!(!sig.verify(msg_b, key_pair.public_key()));
    Ok(())
}
```

## Todo List

Below, in no particular order, I outline what work needs to be completed
on this library.

1. ☑ Make it work - It now works.
2. ☐ Cross-platform builds - The official sqisign library supports many platforms,
   and this crate only supports one.
3. ☐ Sort out GMP - The official sqisign library uses big numbers.  This crate
   makes the choice to use the minigmp library, which may or may not be the
   best or most performant option.
4. ☐ New functions! - As I wrote this, a new function just dropped which supports
   getting _only_ the signature from the official library, which this crate does
   manually.  These are only available in PR 7, which we use.
5. ☐ Verify Safety - This library needs to be looked over by experts.
