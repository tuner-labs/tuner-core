.PHONY: help build test test-all fmt clippy doc clean check release coverage

help:
	@echo "Available targets:"
	@echo "  build       - Build the project"
	@echo "  test        - Run all tests"
	@echo "  test-all    - Run all tests with output"
	@echo "  test-lib    - Run library tests"
	@echo "  test-integ  - Run integration tests"
	@echo "  fmt         - Format code"
	@echo "  clippy      - Run clippy linter"
	@echo "  doc         - Generate and open documentation"
	@echo "  check       - Check code without building"
	@echo "  clean       - Clean build artifacts"
	@echo "  release     - Build release binary"
	@echo "  coverage    - Generate test coverage"

build:
	cargo build

test:
	cargo test --lib

test-all:
	cargo test --all-targets -- --nocapture

test-lib:
	cargo test --lib -- --nocapture

test-integ:
	cargo test --test integration_tests -- --nocapture

fmt:
	cargo fmt

fmt-check:
	cargo fmt -- --check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

doc:
	cargo doc --no-deps --open

check:
	cargo check --all-targets --all-features

clean:
	cargo clean

release:
	cargo build --release

coverage:
	cargo tarpaulin --out Html --output-dir coverage

all: fmt clippy test doc

ci: fmt-check clippy test

watch:
	cargo watch -x build -x test