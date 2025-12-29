FROM rust:latest

RUN apt-get update && apt-get install -y \
    git \
    curl \
    build-essential \
    tmux \
    vim \
    && rm -rf /var/lib/apt/lists/8

WORKDIR /workspace