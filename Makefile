# Point d'entrée unique des commandes de développement, partagé par le hook git et la CI.
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

# Affiche le rust-version déclaré dans Cargo.toml ; échoue s'il est introuvable.
msrv-version:
	@v=$$(sed -n 's/^rust-version = "\(.*\)"/\1/p' Cargo.toml); \
	if [ -z "$$v" ]; then echo "msrv-version: rust-version introuvable dans Cargo.toml" >&2; exit 1; fi; \
	echo "$$v"

# Vérifie la compilation avec le rust-version déclaré dans Cargo.toml (requiert rustup).
msrv-check:
	@v=$$($(MAKE) -s msrv-version) || exit 1; \
	cargo +$$v check --all-targets --locked
