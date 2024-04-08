FROM rust:1.43.1 as build

WORKDIR /usr/src/api-service
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY —from=cargo-build /usr/local/cargo/bin/api-service /usr/local/bin/api-service

CMD [“api-service”]