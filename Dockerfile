FROM rust:1.65
WORKDIR /app
COPY . .
RUN cargo build --release
CMD ["./target/release/producer"]
