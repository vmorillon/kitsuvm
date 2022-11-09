#!/usr/bin/env bash

set -e

RUST_LOG=trace cargo run -- -t ../../templates fifo_*
