FROM rust:latest

WORKDIR /usr/src/map-app-base
COPY . .

RUN cargo install sqlx-cli
RUN sqlx migrate run

RUN cargo build

CMD ["cargo", "run"]