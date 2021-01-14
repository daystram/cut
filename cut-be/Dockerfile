FROM rust:1.49 as builder
WORKDIR /build
COPY . ./
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /build/target/release/cut-be /
CMD ["./cut-be"]
