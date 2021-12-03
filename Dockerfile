FROM rust:1.54 as builder
WORKDIR /app
COPY . .
RUN cargo build --release
RUN strip ./target/release/hololive-archiver-worker

FROM rust:1.54 as runner
WORKDIR /app
COPY --from=builder /app/target/release/hololive-archiver-worker ./
CMD ["/app/hololive-archiver-worker"]