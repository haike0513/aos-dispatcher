FROM rust:1.80.1-bullseye AS builder
WORKDIR /usr/src/app
COPY . ./
RUN cargo build --release



FROM debian:bullseye AS prod
RUN  apt-get update && apt-get -y install libpq5 && apt-get -y install ca-certificates

WORKDIR /usr/src/app

COPY ./dispatcher.toml ./

COPY --from=builder /usr/src/app/target/release/aos-dispatcher ./

CMD ["./aos-dispatcher"]