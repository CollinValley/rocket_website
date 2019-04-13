# Select build image, maybe this should be not rustup, shrug
FROM rustlang/rust:nightly

WORKDIR /usr/src/personal_website

COPY . .

ENV ROCKET_PORT=80
RUN cargo build --release

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/personal_website"]
