FROM rust:1.48.0
RUN apt-get update -y
RUN apt-get install -y --no-install-recommends libssl-dev ca-certificates

RUN cargo install actix-web
