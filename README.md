cd the-sqisign
git fetch origin pull/7/head:mybranch
git checkout mybranch
git merge main
git commit -m "Merged."
cd ..
cargo test --release
