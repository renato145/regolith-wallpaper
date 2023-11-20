_default:
  @just --choose

run-nvidia:
  __NV_PRIME_RENDER_OFFLOAD=1 cargo run

checks:
  #!/usr/bin/env bash
  set -x
  cargo check
  cargo check --tests
  cargo clippy --all-targets
  cargo fmt --all -- --check
