#!/bin/bash
source $HOME/.cargo/env
cd /mnt/c/Dev/Repos/SonnerStudio/Sonner-Studio-Language_SSL
cargo build --release
cp target/release/ssl /mnt/c/Dev/Repos/SonnerStudio/Sonner-Studio-Language_SSL/ssl-v8/binaries/ssl-linux-x64
chmod +x /mnt/c/Dev/Repos/SonnerStudio/Sonner-Studio-Language_SSL/ssl-v8/binaries/ssl-linux-x64
echo "Linux binary built successfully!"
