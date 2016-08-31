#! /bin/bash

# exit if a command fails
set -e

triple=x86_64-unknown-linux-gnu

# install curl (needed to install rust)
apt-get update && apt-get install -y sudo curl gdb g++-multilib lib32stdc++6 libssl-dev libncurses5-dev

# install rust and cargo
curl -sSf https://static.rust-lang.org/rustup.sh | sh

# install libsodium
apt-get install -y pkg-config
apt-get update && apt-get install -y libsodium-dev

# cleanup package manager
apt-get remove --purge -y curl && apt-get autoclean && apt-get clean
rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

# prepare dir
mkdir /source
