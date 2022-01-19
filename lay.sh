#!/bin/bash

# Build and copy to local bin
cargo b --release
mkdir -p ~/.local/bin
cp -v target/release/rename_here ~/.local/bin/

