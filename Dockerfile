FROM rustlang/rust:nightly as builder

COPY . .

RUN cargo build --release

FROM rustlang/rust:nightly

COPY --from=builder /target/release/rust-autocd-builder /bin
