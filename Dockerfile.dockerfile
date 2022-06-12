FROM ubuntu:focal

# RUN cargo install cargo-edit

ENV TZ=Europe/Athens
ENV DEBIAN_FRONTEND=noninteractive

RUN apt update
RUN apt install software-properties-common -y
RUN add-apt-repository ppa:ethereum/ethereum
RUN apt update
RUN apt install build-essential pkg-config libssl-dev git curl solc -y

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
# npm
# RUN npm install solc

ENTRYPOINT [ "target/debug/hello-block" ]
