# Dockerfile for axum backend
# Development stage
FROM rust:1.83 AS development
WORKDIR /app
COPY . /app
RUN cargo build
CMD ["cargo", "run"]

# Production stage
FROM rust:1.83 AS production
WORKDIR /app
COPY . /app
RUN cargo build --release
CMD ["./target/release/backend"]