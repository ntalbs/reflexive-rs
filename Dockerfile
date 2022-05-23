FROM rust:1.61.0
COPY . /app
WORKDIR /app
RUN cargo build --release
EXPOSE 3000
CMD ["target/release/reflexive"]
