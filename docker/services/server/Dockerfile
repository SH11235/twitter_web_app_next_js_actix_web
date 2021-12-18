FROM rust:1.54

WORKDIR /usr/src/twittersearch/server
RUN cargo install diesel_cli --no-default-features --features postgres
EXPOSE 8000

CMD cargo run
