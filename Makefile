all: build
	cargo run --bin advent_of_code

all-slow: build
	cargo run --bin advent_of_code -- --include-slow

build:
	cargo build

scaffold:
	cargo run --bin scaffold ${DAY}

format:
	cargo fmt

check:
	cargo check
	cargo fmt --check
