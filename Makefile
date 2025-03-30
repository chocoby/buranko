.PHONY: build test clean run check format lint

all: build

build:
	cargo build

release:
	cargo build --release

test:
	cargo test

clean:
	cargo clean

run:
	cargo run

check:
	cargo check

format:
	cargo fmt

lint:
	cargo clippy

update:
	cargo update

install:
	cargo install --path .

uninstall:
	cargo uninstall buranko
