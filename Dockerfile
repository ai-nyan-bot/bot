FROM rust:1.85.1-bullseye AS base
RUN cargo install cargo-chef --locked
RUN rustup component add rustfmt
RUN rustup update
RUN apt-get update && apt-get -y install clang cmake
WORKDIR /build

FROM base AS plan
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base AS build
COPY --from=plan /build/recipe.json .
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bins

FROM debian:bullseye-slim AS run
RUN apt-get update && apt-get -y install ca-certificates libc6

COPY --from=build /build/target/release/aggregator          /app/aggregator/aggregator
COPY --from=build /build/target/release/api                 /app/api/api
COPY --from=build /build/target/release/engine              /app/engine/engine
COPY --from=build /build/target/release/indexer             /app/indexer/indexer
COPY --from=build /build/target/release/telegram            /app/telegram/telegram

COPY --from=build /build/bin/aggregator/conf/config.toml    /app/aggregator/config.toml
COPY --from=build /build/bin/api/conf/config.toml           /app/api/config.toml
COPY --from=build /build/bin/engine/conf/config.toml        /app/engine/config.toml
COPY --from=build /build/bin/indexer/conf/config.toml       /app/indexer/config.toml
COPY --from=build /build/bin/telegram/conf/config.toml      /app/telegram/config.toml
COPY --from=build /build/bin/telegram/i18n                  /app/telegram/i18n

RUN adduser --system --group --no-create-home nyan
USER nyan

