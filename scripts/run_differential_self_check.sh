#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

echo "[diff-self-check] building sample reference driver"
cargo build --bin lunar_ref_driver

echo "[diff-self-check] running ignored differential test against local sample driver"
LUNAR_RS_DIFF_REF_BIN="$repo_root/target/debug/lunar_ref_driver" \
LUNAR_RS_DIFF_CASES="$repo_root/tests/fixtures/differential_cases.txt" \
  cargo test diff_reference_sample_matrix -- --ignored
