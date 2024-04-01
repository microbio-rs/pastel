#!/usr/bin/env -S just --justfile

_default:
  @just --list -u

# build cli
build:
	cargo build --bin paastel -p paastel_cli

# install cli
install:
	#!/usr/bin/env sh
	(cd crates/paastel_cli && cargo install --path . --bin paastel --locked)

# run unit tests
test:
	cargo nextest run 

# Format all files
fmt:
	cargo fmt
	taplo fmt
