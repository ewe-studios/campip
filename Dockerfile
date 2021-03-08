FROM rust:latest as deps

WORKDIR /app
COPY . /app

RUN cargo install

FROM deps as build

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM rust:alpine3.12
COPY --from=build /app/bin/campip /bin/campip

CMD ["/bin/campip"]
