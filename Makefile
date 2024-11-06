fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

test:
	cargo test

build:
	cargo build

run:
	cargo run -- "https://www.example.com/path?query=1"

all: fmt clippy test build