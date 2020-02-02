FROM rust:latest as builder

COPY . .

RUN cargo build --release

FROM rust:latest

COPY --from=builder /target/release/rust-autocd-builder /bin
