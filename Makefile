SHELL = /bin/sh
.DEFAULT_GOAL=all

-include .env

deps:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	# TODO install thrift
.PHONY: deps

thrift:
	./generate_thrift_bindings.sh
.PHONY: thrift

build:
	cargo build
.PHONY: build

test:
	cargo test
.PHONY: test

install: test
	cargo install --path .
.PHONY: install

release:
	cargo build --release
.PHONY: release

all: test
.PHONY: all

everything: thrift all release
.PHONY: everything

#
# Docker
#

up:
	docker run --name omnisci-test-db -d --rm -p 6273-6274:6273-6274 omnisci/core-os-cpu:v5.3.0
.PHONY: up

down:
	docker stop omnisci-test-db
.PHONY: down

docker_builder:
	docker build -f docker/Dockerfile -t build-omnisci-rs .
.PHONY: docker_builder

%.docker: docker_builder
	docker run -i --rm -v ${PWD}:/src build-omnisci-rs make $*
