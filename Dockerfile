FROM rust:1.45 as builder

RUN USER=root cargo new --bin rust-docker-web
WORKDIR ./rust-docker-web
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/rust_docker_web*
RUN cargo build --release


FROM alpine:latest
ARG APP=/usr/src/app

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN addgroup $APP_USER \
    && adduser -g $APP_USER $APP_USER

COPY --from=builder\
     /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-docker-web ${APP}/rust-docker-web

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD /usr/local/bin/started