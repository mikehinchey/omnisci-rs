FROM ubuntu:18.04

RUN apt-get update && \
    apt-get install -y \
        build-essential \
        curl \
        perl

RUN curl --proto '=https' --tlsv1.2 -sSf \
    https://sh.rustup.rs -o installrust.sh && \
    sh installrust.sh -y
ENV PATH+=/root/.cargo/bin

# RUN curl \
#     https://www.apache.org/dyn/closer.cgi?path=/thrift/0.13.0/thrift-0.13.0.tar.gz \
#     -o thrift-0.13.0.tar.gz
# RUN tar xzf thrift-0.13.0.tar.gz

RUN mkdir /src
WORKDIR /src
