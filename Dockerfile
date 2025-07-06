# Production stage
FROM debian:bullseye-slim

WORKDIR /app

# Copy the binary from the builder stage
COPY target/release/jamscan /bin/jamscan

# Set environment variables
ENV PATH="/app:${PATH}"

# Set the entrypoint and default command
ENTRYPOINT ["/bin/jamscan"]
CMD []
