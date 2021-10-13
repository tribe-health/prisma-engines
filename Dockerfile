FROM rust:latest as builder
MAINTAINER Julius de Bruijn <bruijn@prisma.io>

ENV USER root

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

ENV WORKSPACE_ROOT=/usr/src/query-engine
ENV RUST_LOG_FORMAT=devel
ENV RUST_BACKTRACE=1
ENV RUST_LOG=query_engine=debug,quaint=debug,query_core=debug,query_connector=debug,sql_query_connector=debug,prisma_models=debug,engineer=debug
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /usr/src/prisma-engines/
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine

COPY --from=builder /usr/src/prisma-engines/target/x86_64-unknown-linux-musl/release/query-engine /usr/bin/query-engine
COPY --from=builder /usr/src/prisma-engines/target/x86_64-unknown-linux-musl/release/introspection-engine /usr/bin/introspection-engine
COPY --from=builder /usr/src/prisma-engines/target/x86_64-unknown-linux-musl/release/migration-engine /usr/bin/migration-engine

CMD /usr/bin/query-engine --host 0.0.0.0 --enable-playground
