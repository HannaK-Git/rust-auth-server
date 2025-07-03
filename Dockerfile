###############################
# 1. Builder stage
###############################
FROM rustlang/rust:nightly-slim AS builder

RUN apt-get update \
 && apt-get install -y musl-tools clang pkg-config build-essential \
 && rm -rf /var/lib/apt/lists/*

# create a directory for the app
# and set it as the working directory
WORKDIR /app

# cash dependencies
COPY Cargo.toml Cargo.lock ./
# main-phase 
RUN mkdir src && echo 'fn main(){}' > src/main.rs
RUN cargo fetch --locked

# 2. Copy source code
# and build the app
COPY . .
# musl-target  
RUN rustup target add x86_64-unknown-linux-musl
ENV RUSTFLAGS="-C target-cpu=native -C link-arg=-s"
RUN cargo build --release --target x86_64-unknown-linux-musl

###############################
# 2. Minimal runtime-image
###############################
FROM scratch

# binary with the app
# from the builder stage

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/nasa-login-app /nasa-login-app

# static files
# Askama-template engine
# requires static files to be copied
COPY --from=builder /app/static /static


EXPOSE 3000            


WORKDIR /

ENTRYPOINT ["/nasa-login-app"]