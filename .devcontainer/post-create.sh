#!/usr/bin/env bash

set -euxo pipefail

sudo apt-get update
sudo apt-get install -y build-essential gnuplot
sudo apt-get clean
sudo rm -rf /var/lib/apt/lists/*

sudo chown vscode:vscode /workspaces

cargo install --force cargo-make

# Clone `shanten-number`.
pushd /workspaces
git clone https://github.com/tomohxx/shanten-number
pushd shanten-number
tar -xzf data.tar.gz
popd
popd
