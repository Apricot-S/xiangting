#!/usr/bin/env bash

set -euxo pipefail

sudo apt-get update
sudo apt-get install -y gnuplot
sudo apt-get clean
sudo rm -rf /var/lib/apt/lists/*

sudo chown vscode:vscode /workspaces

cargo install --force cargo-make
