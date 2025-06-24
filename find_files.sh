#!/usr/bin/bash

find target/release/build/sqisign-rust-97635a8a0ce3ad20/out/build/src/ | sed 's/.*build/build/' | grep '\.a' | sed '/test/d' | sed 's/.*\/lib//' | sed 's/\.a$//' | sed 's/^/println!("cargo:rustc-link-lib=static:+whole-archive=/' | sed 's/$/");/'
