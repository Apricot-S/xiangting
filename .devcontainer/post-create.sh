#!/usr/bin/env bash

set -euxo pipefail

sudo chown -R vscode:vscode .

sudo apt-get update
sudo apt-get install -y gnuplot
sudo apt-get clean
sudo rm -rf /var/lib/apt/lists/*
