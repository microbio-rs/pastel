#!/usr/bin/env -S just --justfile

_default:
  @just --list -u

# Build cli
build-cli:
	cargo build --bin paastel

# Format all files
fmt:
  cargo fmt
