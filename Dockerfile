FROM rust:1.50.0-buster AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

RUN chmod -x /app/target/release/campip

FROM debian:buster-slim

RUN apt-get update && apt-get install -y libpq5 ca-certificates tzdata && \
    rm -rf /var/lib/apt/lists/* && \
    rm -rf /var/cache/apt/archives/*

COPY --from=builder /app/target/release/campip /bin/campip

RUN chmod +x /bin/campip && \
	chown root:root /bin/campip && \
	groupadd app && useradd -g app app

USER app

EXPOSE 7800

CMD ["/bin/campip", "serve"]
