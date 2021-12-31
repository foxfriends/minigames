FROM rust
WORKDIR /database

# Cache dependencies
RUN cargo install sqlx-cli --no-default-features --features "postgres native-tls"

# Build app
COPY . .

CMD ["sqlx", "database", "setup"]
