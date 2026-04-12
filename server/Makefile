SUITE ?= basic

.PHONY: run check fmt test build-release e2e e2e-all e2e-suite ci

run:
	cargo run

check:
	cargo check

fmt:
	cargo fmt

test:
	cargo test

build-release:
	cargo build --release

e2e:
	python3 scripts/e2e_regression.py basic
	python3 scripts/e2e_regression.py auth
	python3 scripts/e2e_regression.py edge
	python3 scripts/e2e_regression.py lifecycle
	python3 scripts/e2e_regression.py command_session
	python3 scripts/e2e_regression.py database
	python3 scripts/e2e_regression.py listeners

e2e-all:
	python3 scripts/e2e_regression.py all

e2e-suite:
	python3 scripts/e2e_regression.py $(SUITE)

ci:
	cargo fmt --check
	cargo check
	cargo test
	$(MAKE) e2e
