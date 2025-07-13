FROM debian:bullseye-slim as builder

ARG POSTGRES_URI
ENV POSTGRES_URI=${POSTGRES_URI}

# Install build dependencies
RUN apt-get update && apt-get install -y \
    gnupg \
    curl \
    build-essential \
    libpq-dev \
    pkg-config \
    && apt-get clean

# Install Rust with automatic approval of installation
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Cargo's bin directory to the PATH
ENV PATH=/root/.cargo/bin:$PATH

# Verify if cargo is available and install diesel_cli
RUN cargo --version && cargo install diesel_cli --no-default-features --features postgres

# Use the builder as the base for your app image
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    && apt-get clean

RUN mkdir -p db/migrations

# Copy the diesel binary from the builder image
COPY --from=builder /root/.cargo/bin/diesel /usr/local/bin/diesel
COPY db/migrations/ db/migrations/

# Use a shell command to dynamically expand POSTGRES_URI
ENTRYPOINT ["/bin/sh", "-c", "diesel migration run --migration-dir /app/db/migrations/stock --database-url \"$POSTGRES_URI\""]