# Production stage
FROM --platform=linux/amd64 debian:bullseye-slim

WORKDIR /app

# Copy the binary from the builder stage
COPY jamscan /app/jamscan

# Set environment variables
ENV PATH="/app:${PATH}"

# Set the entrypoint and default command
ENTRYPOINT ["/app/jamscan"]
CMD []
