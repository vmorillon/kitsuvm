#!/usr/bin/env bash

set -e

RUST_LOG=trace cargo run -- --no-top -t ../../templates healthcheck.toml
