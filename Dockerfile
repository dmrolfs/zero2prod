FROM rust:1.47
WORKDIR app
ENV SQLX_OFFLINE true
COPY . .
RUN cargo build --release
ENTRYPOINT ["./target/release/zero2prod"]
