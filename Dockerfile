FROM rust:1.81

WORKDIR /app

COPY . .

RUN cargo install sea-orm-cli --version 1.0.1

RUN cargo install cargo-watch --version 8.5.2

CMD ["cargo", "watch", "-w", "/app", "-x", "run --release"]
