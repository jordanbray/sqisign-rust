#!/usr/bin/bash

# You probably don't need this code.  It is only here for documentation
# purposes.  The `build.rs` file has already been made, and is in the
# repository.
# 
# This script finds all the `.a` files associated with the built
# version of this project (after a `cargo build --release`), and then
# reformats the text to be compatibile with the `build.rs` file rust
# wants.
#
# WARNING: this does _not_ find the GMP library.  If you build your
# own `build.rs`, you will need to manually add GMP somehow.

find target/release/build/sqisign-rust-*/out/build/src/ |
  sed 's/.*build/build/' |
  grep '\.a' |
  sed '/test/d' |
  sed 's/.*\/lib//' |
  sed 's/\.a$//' |
  sed 's/^/println!("cargo:rustc-link-lib=static:+whole-archive=/' |
  sed 's/$/");/'
