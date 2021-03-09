FROM rust:1.50.0-buster AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y ca-certificates tzdata && rm -rf /var/lib/apt/lists/*

RUN groupadd app && useradd -g app app

COPY --chown=app --from=builder /app/target/release/campip /bin/campip

RUN chmod -x /bin/campip

#USER app

EXPOSE 7800

CMD ["/bin/campip"]
