all: build run

build:
	cargo build

run:
	cargo run

format:
	cargo fmt

check:
	cargo check
	cargo fmt --check
