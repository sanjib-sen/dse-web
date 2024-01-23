# Stage 1: Build the application
FROM rust:latest as builder

WORKDIR /app
RUN cargo install dioxus-cli
RUN rustup target add wasm32-unknown-unknown
RUN apt-get update && apt-get install -y ca-certificates pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN update-ca-certificates
COPY . .
RUN dx build --features web --release
RUN cargo build --features ssr --release

FROM debian:testing-slim
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

WORKDIR /app
COPY --from=builder /app/target/release/dse-web .
COPY --from=builder /app/dist ./dist

EXPOSE 8080 
CMD [ "./dse-web" ]
