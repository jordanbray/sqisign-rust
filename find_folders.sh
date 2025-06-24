#!/usr/bin/bash

# You probably don't need this code.  It is only here for documentation
# purposes.  The `build.rs` file has already been made, and is in the
# repository.
# 
# This script finds all the folders that the various `.a` files
# ended up in, so that the linker can locate them all.  It then
# outputs them in a way that is compatible with the `build.rs` file
# from rust.
#
# 
# WARNING: this does _not_ find the GMP library.  If you build your
# own `build.rs`, you will need to manually add GMP somehow.

find target/release/build/sqisign-rust-*/out/build/src/ |
  sed 's/.*build/\/build/' |
  grep '\.a' |
  sed '/test/d' |
  sed 's/\/[a-z_.0-9]*$//' |
  sort |
  uniq |
  sed 's/^/println!("cargo:rustc-link-search=native={}/' |
  sed 's/$/", dst.display());/'
