FROM rust

WORKDIR /server
COPY src/* src/*
COPY Cargo.toml /server/Cargo.toml
RUN cargo build -r

ENTRYPOINT [ "cargo", "r", "-r" ]