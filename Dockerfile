FROM rust as builder

COPY config ~/.cargo

WORKDIR app
COPY . .

RUN cargo build --release --bin wiki-graph

FROM rust as runtime
WORKDIR app
COPY --from=builder /app/target/release/wiki-graph /usr/local/bin

EXPOSE 3690

CMD ["wiki-graph","-d","app"]