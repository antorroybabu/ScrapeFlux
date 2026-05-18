# ScrapeFlux Dockerfile - Multi-stage build for optimal size
# Author: Antor Roy
# Email: antorroybabu@gmail.com

# =============================================================================
# Stage 1: Builder
# =============================================================================
FROM rust:1.70-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy manifests first for dependency caching
COPY Cargo.toml Cargo.lock* ./
COPY src ./src

# Create dummy main.rs for dependency compilation
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Build dependencies only
RUN cargo build --release && rm -rf src

# Copy actual source
COPY src ./src
COPY python ./python

# Build the library
RUN cargo build --release --lib

# =============================================================================
# Stage 2: Python runtime
# =============================================================================
FROM python:3.11-slim AS python_deps

WORKDIR /app

# Install Python dependencies
COPY deploy/requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# =============================================================================
# Stage 3: Runtime
# =============================================================================
FROM python:3.11-slim AS runtime

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    gnupg \
    && curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 scrapeflux

# Set working directory
WORKDIR /app

# Copy Python dependencies from stage 2
COPY --from=python_deps /usr/local/lib/python3.11/site-packages /usr/local/lib/python3.11/site-packages
COPY --from=python_deps /usr/local/bin /usr/local/bin

# Copy Rust library
COPY --from=builder /app/target/release/libscrapeflux.so /usr/local/lib/
COPY --from=builder /app/target/release/libscrapeflux.rlib /usr/local/lib/

# Set library path
ENV LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH

# Copy application code
COPY python ./python
COPY deploy ./deploy
COPY examples ./examples
COPY configs ./configs

# Create data directories
RUN mkdir -p /data/cache /data/logs /data/output && \
    chown -R scrapeflux:scrapeflux /data

# Switch to non-root user
USER scrapeflux

# Set environment variables
ENV SCRAPEFLUX_HOME=/app
ENV SCRAPEFLUX_DATA=/data
ENV SCRAPEFLUX_LOG_LEVEL=INFO
ENV PYTHONPATH=/app/python

# Expose ports
EXPOSE 8000 8080 9090

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/health || exit 1

# Default command
CMD ["python", "-m", "scrapeflux", "serve", "--host", "0.0.0.0", "--port", "8000"]

# =============================================================================
# Metadata
# =============================================================================
LABEL maintainer="Antor Roy <antorroybabu@gmail.com>"
LABEL description="ScrapeFlux - Ultra-Advanced Web Scraping Framework"
LABEL version="1.0.0"
LABEL homepage="https://github.com/antorroy/scrape-flux"
LABEL documentation="https://scrape-flux.readthedocs.io"