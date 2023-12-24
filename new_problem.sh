#!/usr/bin/env bash
set -euo pipefail

problem_name=$1

last_number=$(ls -d [0-9][0-9]-* | sed 's/-.*//' | tail -1)
next_number=$((last_number + 1))
dir_name="${next_number}-${problem_name}"

cargo new --lib "$problem_name"
mv "$problem_name" "$dir_name"

# Cargo.toml の最終行に作成したディレクトリ名を追加する
gsed -i "$(wc -l < Cargo.toml)i\  \"${dir_name}\",
" Cargo.toml
