FROM rust:1.83

ENV RUST_BACKTRACE=1

# Set the working directory
WORKDIR /app

# Install PostgreSQL client libraries and other dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*


RUN apt-get update && apt-get install -y postgresql-client

# Copy Rust project files and fetch dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked

# Copy the rest of the application
COPY . .

# Expose the application port
EXPOSE 3000

# Command to run the application
CMD ["cargo", "run"]
