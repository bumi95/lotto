FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cd webserver && cargo build --release && mv ./target/release/webserver ./webserver

FROM ubuntu:24.04
RUN apt-get update && apt-get install -y openssl ca-certificates
RUN apt-get install -y libsqlite3-0

RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

COPY --from=builder /usr/src/app/webserver/webserver /app/webserver
COPY --from=builder /usr/src/app/webserver/lotto.db /app/lotto.db
COPY --from=builder /usr/src/app/ui/build /app/ui/build

CMD ./webserver
