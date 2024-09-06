FROM lukemathwalker/cargo-chef:latest-rust-alpine AS chef
WORKDIR /RetroBot

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN apk add --no-cache curl
COPY --from=planner /RetroBot/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin RetroBot

# We do not need the Rust toolchain to run the binary!
FROM alpine:3.20 AS runtime
WORKDIR /RetroBot
COPY --from=builder /RetroBot/target/release/RetroBot /usr/local/bin
ENTRYPOINT ["/usr/local/bin/RetroBot"]
