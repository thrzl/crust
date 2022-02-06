FROM rust:latest

COPY ./ /crust
WORKDIR /crust

RUN rustup install nightly
RUN rustup default nightly
RUN cargo run --release