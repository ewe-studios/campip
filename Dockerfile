FROM rust:1.50.0-buster AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y ca-certificates tzdata && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/campip /bin/campip
RUN chmod -x /bin/campip

EXPOSE 7800

RUN groupadd app && useradd -g app app

USER app
RUN chown -R app:app /bin/campip

CMD ["/bin/campip"]
