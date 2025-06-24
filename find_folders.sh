#!/usr/bin/bash

find target/release/build/sqisign-rust-*/out/build/src/ | sed 's/.*build/\/build/' | grep '\.a' | sed '/test/d' | sed 's/\/[a-z_.0-9]*$//' | sort | uniq | sed 's/^/println!("cargo:rustc-link-search=native={}/' | sed 's/$/", dst.display());/'
