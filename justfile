_default:
  @just --choose

run-nvidia-prime:
  __NV_PRIME_RENDER_OFFLOAD=1 cargo run -- 30

run-nvidia:
  WGPU_BACKEND=gl cargo run -- 30

checks:
  #!/usr/bin/env bash
  set -x
  cargo check
  cargo check --tests
  cargo clippy --all-targets
  cargo fmt --all -- --check
