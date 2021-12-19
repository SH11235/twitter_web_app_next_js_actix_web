FROM rust:1.57

USER 1000:1000
WORKDIR /usr/src/twittersearch/server
RUN cargo install diesel_cli --no-default-features --features postgres
EXPOSE 8000

CMD cargo run
