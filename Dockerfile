FROM lukemathwalker/cargo-chef:latest-rust-bullseye AS chef
WORKDIR /crust

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /crust/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin crust

FROM debian:bullseye-slim AS runtime
WORKDIR /crust
COPY --from=builder /crust/target/release/crust /usr/local/bin
RUN apt-get update
RUN apt-get install ca-certificates -y
ENTRYPOINT ["/usr/local/bin/crust"]
