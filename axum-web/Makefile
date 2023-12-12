#export RUSTFLAGS=-Aunused_variables -Aunused_imports -Adead_code -Aunused_must_use
export RUST_LOG=debug
install:
	cargo build

build:
	cargo build

dev: ## run development env with hot reload
	cargo watch -x run

run:
	cargo run