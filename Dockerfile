# Select build image, maybe this should be not rustup, shrug
FROM rustlang/rust:nightly

WORKDIR /usr/src/personal_website

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN mkdir src/

RUN echo "fn main() {println!(\"If you see this, the build broke\")}" > src/main.rs

ENV ROCKET_PORT=80
RUN cargo build --release

RUN rm -f target/release/deps/personal_website*

COPY . .

ENV ROCKET_PORT=80
RUN cargo build --release
RUN cargo install --path .

CMD ["/usr/local/cargo/bin/personal_website"]
