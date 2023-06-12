FROM rust:latest

WORKDIR /usr/src/actix-forum
COPY . .

RUN cargo install cargo-watch
CMD cargo watch -x run
