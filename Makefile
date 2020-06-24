#
# Simple Makefile to call the rust and cargo commands
#

SHELL = /bin/sh
.DEFAULT_GOAL=all

-include .env

deps:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

build:
	cargo build

test:
	cargo test

install: test
	cargo install --path .

release:
	cargo build --release

all: test

.PHONY: dev build test all install release
