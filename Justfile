#!/usr/bin/env -S just --justfile

_default:
  @just --list -u

# Build cli
build:
	cargo build --bin paastel -p paastel_cli

# Install cli
install:
	#!/usr/bin/env sh
	(cd crates/paastel_cli && cargo install --path . --bin paastel --locked)


# Format all files
fmt:
  cargo fmt
  taplo fmt
