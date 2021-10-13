FROM rust:latest as builder
MAINTAINER Julius de Bruijn <bruijn@prisma.io>

ENV USER root

RUN rustup target add x86_64-unknown-linux-musl

RUN apt-get -y update
RUN apt-get -y install libssl-dev build-essential

ENV WORKSPACE_ROOT=/usr/src/query-engine
ENV RUST_LOG_FORMAT=devel
ENV RUST_BACKTRACE=1
ENV RUST_LOG=query_engine=debug,quaint=debug,query_core=debug,query_connector=debug,sql_query_connector=debug,prisma_models=debug,engineer=debug
ENV PATH="/root/.cargo/bin:${PATH}"

ADD . /usr/src/prisma-engines
WORKDIR /usr/src/prisma-engines/

RUN cargo build --release
RUN cargo install --target x86_64-unknown-linux-musl --path ./bin

FROM alpine:latest

COPY --from=builder /usr/src/prisma-engines/bin/query-engine /usr/bin/query-engine
COPY --from=builder /usr/src/prisma-engines/bin/introspection-engine /usr/bin/introspection-engine
COPY --from=builder /usr/src/prisma-engines/bin/migration-engine /usr/bin/migration-engine

CMD /usr/bin/query-engine --host 0.0.0.0 --enable-playground
