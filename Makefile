# Single entry point for development commands, shared by the git hook and the CI.
.PHONY: setup format fmt-check lint quality test-unit test msrv-version msrv-check

setup:
	git config core.hooksPath .githooks

format:
	cargo fmt --all

fmt-check:
	cargo fmt --all --check

lint:
	cargo clippy --all-targets -- -D warnings

quality: fmt-check lint
	.agents/scripts/compile-agents --check

test-unit:
	cargo test --lib
	cargo test --doc

test: test-unit

# Prints the rust-version declared in Cargo.toml; fails if it is missing.
msrv-version:
	@v=$$(sed -n 's/^rust-version = "\(.*\)"/\1/p' Cargo.toml); \
	if [ -z "$$v" ]; then echo "msrv-version: rust-version not found in Cargo.toml" >&2; exit 1; fi; \
	echo "$$v"

# Checks that the crate compiles with the rust-version declared in Cargo.toml (requires rustup).
msrv-check:
	@v=$$($(MAKE) -s msrv-version) || exit 1; \
	cargo +$$v check --all-targets --locked
