FROM rust AS builder
WORKDIR /server

# Build app
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo install --path .

# Copy to runtime image
FROM debian:bullseye-slim
EXPOSE 8000
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
RUN apt-get update && apt-get install ca-certificates -y && apt-get clean

CMD ["server"]
