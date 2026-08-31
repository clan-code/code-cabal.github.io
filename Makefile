.PHONY: setup dev build check test validate clean

setup:
	rustup target add wasm32-unknown-unknown
	cargo install trunk --locked --version 0.21.14

dev:
	trunk serve --open

build:
	trunk build --release

validate:
	python3 scripts/validate_content.py

check: validate
	cargo fmt --all -- --check
	cargo clippy --lib --all-targets -- -D warnings
	trunk build --release

test:
	cargo test --lib

clean:
	cargo clean
	rm -rf dist
