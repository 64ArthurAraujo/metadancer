sudo rm -r /bin/metadancer

cargo build -r

sudo cp -r target/release/metadancer /bin/
