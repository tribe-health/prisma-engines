FROM rust:latest as builder
MAINTAINER Julius de Bruijn <bruijn@prisma.io>

ENV USER root

RUN apt-get -y update
RUN apt-get -y install libssl-dev build-essential

ENV WORKSPACE_ROOT=/usr/src/query-engine
ENV RUST_LOG_FORMAT=devel
ENV RUST_BACKTRACE=1
ENV RUST_LOG=query_engine=debug,quaint=debug,query_core=debug,query_connector=debug,sql_query_connector=debug,prisma_models=debug,engineer=debug
ENV PATH="/root/.cargo/bin:${PATH}"

ADD . /usr/src/query-engine
WORKDIR /usr/src/query-engine/

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /usr/src/query-engine/target/release/query-engine /usr/bin/query-engine

CMD /usr/bin/query-engine --host 0.0.0.0 --enable-playground
