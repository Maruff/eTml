# Use the official Rust image as a base
FROM rust:latest

# Install ANTLR
RUN apt-get update && \
    apt-get install -y default-jdk wget && \
    wget https://www.antlr.org/download/antlr-4.9.3-complete.jar -P /usr/local/lib/ && \
    echo 'alias antlr4="java -jar /usr/local/lib/antlr-4.9.3-complete.jar"' >> ~/.bashrc && \
    . ~/.bashrc

# Install LLVM dependencies
RUN apt-get install -y llvm clang

# Install Rust tools and components
RUN rustup component add llvm-tools-preview
RUN cargo install cargo-binutils

# Install the ANTLR4 Rust runtime
RUN cargo install antlr4-rust

# Set the working directory
WORKDIR /workspace

# Copy the current project
COPY . .

# Build the project
RUN cargo build
