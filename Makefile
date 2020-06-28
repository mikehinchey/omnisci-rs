SHELL = /bin/sh
.DEFAULT_GOAL=all

-include .env

deps:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	# TODO install thrift
.PHONY: deps

thrift:
	./generate_thrift_bindings.sh ${OMNISCI_PATH}
.PHONY: thrift

build:
	cargo build
.PHONY: build

up:
	docker run --name omnisci-test-db -d --rm -p 6273-6274:6273-6274 omnisci/core-os-cpu:v5.3.0
.PHONY: up

down:
	docker stop omnisci-test-db
.PHONY: down

test:
	cargo test
.PHONY: test

install: test
	cargo install --path .
.PHONY: install

release:
	cargo build --release
.PHONY: release

everything: thrift all release
.PHONY: everything

docker_builder:
	# -q
	docker build -f docker/Dockerfile -t build-omnisci-rs .
.PHONY: docker_builder

%.docker: docker_builder
	docker run -i --rm -v ${PWD}:/src build-omnisci-rs make $*

all: test
.PHONY: all
