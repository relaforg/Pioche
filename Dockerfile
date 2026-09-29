FROM rust:1-trixie AS base
WORKDIR /app

RUN wget -qO- https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
  | tar -xzf - -C /usr/local/cargo/bin
RUN cargo binstall cargo-leptos -y
RUN rustup target add wasm32-unknown-unknown

FROM base AS dev
CMD ["cargo", "leptos", "watch"]

FROM base AS build
COPY . .
RUN cargo leptos build --release -vv

FROM debian:trixie-slim AS prod
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# Copy the server binary to the /app directory
COPY --from=build /app/target/release/pioche /app/

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=build /app/target/site /app/site

# Copy Cargo.toml if it’s needed at runtime
# COPY --from=builder /app/Cargo.toml /app/

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_ENV="PROD"
EXPOSE 8080

# Run the server
CMD ["/app/pioche"]
