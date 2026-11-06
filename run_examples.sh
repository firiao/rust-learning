#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

out_dir="${TMPDIR:-/tmp}/rust-learning-examples"
mkdir -p "$out_dir"

for source in examples/*.rs; do
    name="$(basename "$source" .rs)"
    printf '\n==> %s\n\n' "$name"
    rustc --edition 2024 -o "$out_dir/$name" "$source"
    "$out_dir/$name"
done

printf '\nГотово: все примеры собраны и запущены.\n'
