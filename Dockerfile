FROM rust:1.54 as builder
WORKDIR /app
COPY . .
RUN cargo build --release
RUN strip ./target/release/hololive-archiver-worker

FROM rust:1.54 as runner
WORKDIR /app
RUN apt-get update
RUN apt-get install -y python3-pip
RUN pip3 install yt-dlp
COPY --from=builder /app/target/release/hololive-archiver-worker ./
CMD ["/app/hololive-archiver-worker"]