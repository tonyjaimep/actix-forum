FROM rust:latest

WORKDIR /usr/src/actix-forum
COPY . .

RUN cargo install cargo-watch
RUN cargo install diesel_cli
CMD cargo watch -x run
