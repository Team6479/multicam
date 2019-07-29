FROM rust:latest

WORKDIR /usr/src/multicam-server
COPY . .

RUN cargo install --path .

CMD ["multicam-server"]