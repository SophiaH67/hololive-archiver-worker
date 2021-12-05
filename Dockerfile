FROM rust:1.54 as builder
WORKDIR /app
COPY . .
RUN cargo build --release
RUN strip ./target/release/hololive-archiver-worker

FROM rust:1.54 as ytarchive-downloader
WORKDIR /app
RUN wget https://github.com/Kethsar/ytarchive/releases/download/v0.3.0/ytarchive_linux_amd64.zip
RUN unzip ytarchive_linux_amd64.zip
RUN rm ytarchive_linux_amd64.zip
RUN chmod +x ./ytarchive

FROM rust:1.54 as runner
WORKDIR /app
RUN apt-get update
RUN apt-get install -y python3-pip ffmpeg
RUN pip3 install yt-dlp
COPY --from=builder /app/target/release/hololive-archiver-worker ./
COPY --from=ytarchive-downloader /app/ytarchive /usr/local/bin/ytarchive
CMD ["/app/hololive-archiver-worker"]