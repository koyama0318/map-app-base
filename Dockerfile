FROM rust:latest

WORKDIR /usr/src/map-app-base
COPY . .

RUN cargo install --path .
RUN cargo run

CMD ["map-app-base"]