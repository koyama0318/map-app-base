FROM rust:latest

RUN cargo install sqlx-cli
RUN sqlx migrate run

RUN cargo build

CMD ["cargo", "run"]