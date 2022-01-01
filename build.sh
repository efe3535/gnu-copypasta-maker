#!/bin/sh

cargo build --release
echo "Copying binary to /usr/bin"
sudo cp target/release/gnu-copypasta-maker /usr/bin && echo "Copied succesfully."
