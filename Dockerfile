FROM rust:1.81

WORKDIR /app

COPY . .

RUN cargo install cargo-watch

CMD ["cargo", "watch", "-w", "/app", "-x", "run --release"]
