FROM rust:latest as frontend_builder

WORKDIR /app
RUN cargo install dioxus-cli
RUN rustup target add wasm32-unknown-unknown
COPY . .
RUN dx build --features web --release

FROM rust:alpine AS backend_builder
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup toolchain install nightly-x86_64-unknown-linux-musl
RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-musl
RUN apk add --no-cache musl-dev upx pkgconfig openssl-dev make perl
WORKDIR /app
COPY . .
RUN cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort \
    --target x86_64-unknown-linux-musl --features ssr --release
RUN upx --best --lzma target/x86_64-unknown-linux-musl/release/dse-web
COPY --from=frontend_builder /app/dist ./dist
# RUN wasm-opt /app/dist/assets/dioxus/SENX_DSE_bg.wasm -o /app/dist/assets/dioxus/SENX_DSE_bg.wasm -Oz // install binaryen

FROM scratch
COPY --from=backend_builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
WORKDIR /app
COPY --from=backend_builder /app/target/x86_64-unknown-linux-musl/release/dse-web .
COPY --from=backend_builder /app/dist ./dist
EXPOSE 8080 
CMD [ "./dse-web" ]
