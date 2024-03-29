FROM rust:1.77.0 as builder

# Create a new Rust project
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

# Remove the dummy build artifacts; Note: Adjust the target name if necessary
RUN rm ./target/release/deps/api* && \
    cargo build --release


# Production image
FROM debian:10.13-slim
ARG APP=/app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates tzdata && \
    rm -rf /var/lib/apt/lists/* && \
    mkdir -p ${APP}

WORKDIR ${APP}

# Copy the binary from the builder stage
COPY --from=builder /api/target/release/api .

# Use an unprivileged user for better security
RUN groupadd -r rustuser && \
    useradd -r -g rustuser rustuser && \
    chown rustuser:rustuser ./
USER rustuser

CMD ["./api"]
