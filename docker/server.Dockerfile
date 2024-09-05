FROM rust:1.80.1-alpine

# TODO: finish it

RUN rustup target add x86_64-unknown-linux-gnu
RUN cargo build --target x86_64-unknown-linux-gnu --release

RUN echo "TODO!"
