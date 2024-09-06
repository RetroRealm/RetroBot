FROM lukemathwalker/cargo-chef:latest-rust-alpine AS chef
WORKDIR /retrobot

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN apk add --no-cache curl
COPY --from=planner /retrobot/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin retrobot

# We do not need the Rust toolchain to run the binary!
FROM alpine:3.20 AS runtime
WORKDIR /retrobot
COPY --from=builder /retrobot/target/release/retrobot /usr/local/bin
ENTRYPOINT ["/usr/local/bin/retrobot"]
