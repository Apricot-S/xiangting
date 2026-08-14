FROM mcr.microsoft.com/devcontainers/rust:2-1-trixie

RUN apt-get update && \
    apt-get install -y --no-install-recommends gnuplot && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
