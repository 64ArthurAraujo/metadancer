rm -r ~/.local/bin/metadancer
cargo build -r
cp -r target/release/metadancer ~/.local/bin/
