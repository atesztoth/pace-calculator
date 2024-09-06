FROM rust:1.80.1-alpine as builder

ARG DIESEL_FEATURES=sqlite

RUN apk add --no-cache \
    gcc \
    g++ \
    musl-dev \
    make \
    libc-dev \
    libgcc \
    sqlite-dev

WORKDIR /app

# https://rust-lang.github.io/rfcs/1721-crt-static.html
RUN RUSTFLAGS="-Ctarget-feature=-crt-static" cargo install diesel_cli --debug --no-default-features --features "${DIESEL_FEATURES}"

COPY . .

RUN cargo build --release

# ---
FROM alpine:3.19.3 as runner

ARG APP_NAME=pace-calculator

RUN apk add --no-cache \
    gcc \
    g++ \
    musl-dev \
    libgcc \
    sqlite-dev
    #    make \
    #    libc-dev \

WORKDIR /app

# Copy binary from build stage
COPY --from=builder /app/target/release/${APP_NAME} .

# Copy Diesel CLI binary from build stage
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

COPY --from=builder /app/migrations migrations

#RUN ldd /usr/local/bin/diesel

CMD ["/app/pace-calculator"]
