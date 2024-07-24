FROM rust:latest

WORKDIR /usr/src/map-app-base
COPY . .

RUN cargo install --path .

CMD ["map-app-base"]