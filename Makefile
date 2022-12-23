all: build
	cargo test --bins

everything: build
	cargo test --bins -- --include-ignored

one:
	cargo test --bin ${DAY}

one-everything:
	cargo test --bin ${DAY} -- --include-ignored

build:
	cargo build

scaffold:
	cargo run --bin scaffold ${DAY}

format:
	cargo fmt

check:
	cargo check
	cargo fmt --check
