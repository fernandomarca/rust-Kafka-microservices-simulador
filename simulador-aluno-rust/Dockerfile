FROM rust:1.61-slim-buster

WORKDIR /rust/src
ENV PATH="/rust/bin:$PATH"

RUN apt-get update && \ 
  apt-get install -y build-essential \ 
  ca-certificates \ 
  tzdata \
  librdkafka-dev \
  curl \
  protobuf-compiler \ 
  cmake \
  openssl \
  libssl-dev \
  libsasl2-dev \
  pkg-config \
  libpq-dev \
  libsqlite3-dev \
  libzstd-dev

# RUN cargo install -q diesel_cli

CMD ["tail", "-f", "/dev/null"]