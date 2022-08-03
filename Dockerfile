FROM rust:1.62.1 as builder
ENV USER root
ENV TARGET armv7-unknown-linux-gnueabihf #x86_64-unknown-linux-musl
COPY . /home_api
WORKDIR home_api
RUN rustup update && apt update && apt install pkg-config musl-tools && rustup target add  
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu


FROM scratch
COPY --from=builder /home_api/target/x86_64-unknown-linux-gnu/home_api /home_api
CMD ["/home_api"]
