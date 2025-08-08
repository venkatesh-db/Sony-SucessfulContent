
# ---------- Build Stage ----------
FROM rustlang/rust:nightly-slim AS builder

WORKDIR /app

# ✅ Install required tools for building native Rust dependencies (like rdkafka-sys)
RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev cmake zlib1g-dev g++ \
 && apt-get clean && rm -rf /var/lib/apt/lists/*

# Copy source
COPY . .

# Clear old lock and generate fresh with nightly support
RUN rm -f Cargo.lock && cargo +nightly generate-lockfile

# ✅ Build using edition2024 (requires nightly)
RUN cargo +nightly build --release

# ---------- Runtime Stage ----------
FROM debian:bookworm-slim

# ✅ Install only required runtime dependencies (libssl3 and certs)
RUN apt-get update && apt-get install -y \
    libssl3 ca-certificates \
 && apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy compiled binary from builder
COPY --from=builder /app/target/release/npci_txn_api .

EXPOSE 8080
CMD ["./npci_txn_api"]
