FROM rust:1.63 as builder

RUN USER=root cargo new --bin rust_time_api
WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/api*
RUN cargo build --release


# Production image
FROM debian:10.13-slim
ARG APP=/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/* \
    && mkdir -p ${APP}

WORKDIR ${APP}

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/api .

# Use an unprivileged user
RUN groupadd -r rustuser && useradd -r -g rustuser rustuser
USER rustuser

CMD ["./api"]
