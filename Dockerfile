FROM ubuntu:jammy

RUN apt-get update && apt-get install -y ca-certificates curl gnupg libssl-dev pkg-config tar wget vim less nodejs npm
RUN mkdir /root/worksop/
WORKDIR /root/worksop/
ENV DEBIAN_FRONTEND=noninteractive

RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs > rustup.sh && sh rustup.sh -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup component add rust-src && cargo install --force --locked cargo-contract
