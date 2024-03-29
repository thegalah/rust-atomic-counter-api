FROM rust:1.77.0 as builder

RUN USER=root cargo new --bin api
WORKDIR /api

# Copy your Cargo.toml and Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# Trick to cache dependencies (without creating src since it's already there)
RUN echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm src/main.rs

# Now copy your actual source code
COPY ./src ./src

# Build your Rust application targeting musl for static linking
RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl

# Production image
FROM alpine:3.19.1
ARG APP=/app

# Install runtime dependencies if any
RUN apk add --no-cache ca-certificates tzdata

WORKDIR ${APP}

# Copy the binary from the builder stage and ensure it's statically linked
COPY --from=builder /api/target/x86_64-unknown-linux-musl/release/api .

# Use an unprivileged user for better security
RUN addgroup -S rustuser && adduser -S rustuser -G rustuser && \
    chown rustuser:rustuser ./
USER rustuser

CMD ["./api"]