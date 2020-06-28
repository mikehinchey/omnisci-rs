FROM thrift/thrift-build:ubuntu-bionic AS thrift-build

ARG THRIFT_VERSION=0.13.0
RUN curl \
    http://apache.claz.org/thrift/${THRIFT_VERSION}/thrift-${THRIFT_VERSION}.tar.gz \
    -o thrift-${THRIFT_VERSION}.tar.gz
# get MD5 from https://thrift.apache.org/download
RUN echo "38a27d391a2b03214b444cb13d5664f1  thrift-0.13.0.tar.gz" | md5sum -c
RUN tar xzf thrift-${THRIFT_VERSION}.tar.gz &&\
    rm thrift-${THRIFT_VERSION}.tar.gz

WORKDIR /thrift/src/thrift-${THRIFT_VERSION}
RUN ./configure
RUN make
RUN make install

########

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


RUN mkdir /src
WORKDIR /src
