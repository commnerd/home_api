FROM rust:1.62.1 as builder
ENV USER root
COPY . /home_api
WORKDIR home_api
RUN cargo build --release

FROM scratch
COPY --from=builder /home_api/target/release/home_api /home_api
CMD ["/home_api"]
