#!/usr/bin/env bash

set -euxo pipefail

sudo apt-get update
sudo apt-get install -y build-essential gnuplot
sudo apt-get clean
sudo rm -rf /var/lib/apt/lists/*

sudo chown vscode:vscode /workspaces

chmod +x scripts/*.sh

# Clone `nyanten`.
pushd /workspaces
git clone https://github.com/Cryolite/nyanten
popd
