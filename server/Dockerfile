FROM rust:bullseye AS builder
WORKDIR /server

# Builder also requires Node JS
RUN apt-get update -yq \
    && apt-get install nodejs npm -yq \
    && apt-get clean -y

# Build app
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo install --path .

# Copy to runtime image
FROM debian:bullseye-slim
EXPOSE 8000

COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
COPY --from=builder /server/public /public
RUN apt-get update && apt-get install ca-certificates -y && apt-get clean

CMD ["server"]
