#-----------------------------
#   BUILD STAGE
#-----------------------------

FROM rust:latest AS base

ENV USER=root

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch

COPY src /code/src

CMD [ "cargo", "test", "offline"]

FROM base AS builder

RUN cargo build --release

#---------------------------------
#   FINAL STAGE
#---------------------------------

FROM gcr.io/distroless/cc-debian10

COPY --from=builder /code/target/release/started /usr/bin/started

ENV RUSTFLAGS=-Awarnings

EXPOSE 3000
# EXPOSE 9090

ENTRYPOINT [ "/usr/bin/started" ]
