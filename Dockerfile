# Builder stage
FROM rust:1-bullseye AS builder

WORKDIR /usr/src/app

# Install SSH and git
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    openssh-client \
    git

# Set up SSH for GitHub
# RUN mkdir -p /root/.ssh && \
#     chmod 700 /root/.ssh && \
#     ssh-keyscan github.com >> /root/.ssh/known_hosts
# 
# # Copy SSH key for private repos
# COPY .ssh/id_ed25519 /root/.ssh/id_ed25519
# RUN chmod 600 /root/.ssh/id_ed25519

# Copy the actual source code
COPY . .

# Build the release binary
RUN cargo build --release

# Production stage
FROM debian:bullseye-slim

WORKDIR /app

# Install runtime dependencies with minimal image bloat
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl1.1 \
    && apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/jamscan /app/jamscan

# Create a non-root user and switch to it
RUN useradd -m jamscan && \
    chown -R jamscan:jamscan /app
USER jamscan

# Set the entrypoint
ENTRYPOINT ["/app/jamscan"]
