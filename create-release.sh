#!/bin/sh
set -e
cargo test
for example in examples/*
do
    example=$(basename $example .rs)
    echo "---- Building example: $example ----"
    cargo +nightly web build --target wasm32-unknown-unknown --release --example $example
    echo "--- Copying build artifacts: $example ----"
    cp target/wasm32-unknown-unknown/release/examples/$example.wasm docs/wasm/$example.wasm
    cp target/wasm32-unknown-unknown/release/examples/$example.js docs/wasm/$example.js
done

echo "--- Copying assets to the web directory ---"
cp static/* docs/
read -p "New version string: " version

echo "--- Bumping version of the html_url_root ---"
sed -e "s/html_root_url = \".*\"/html_root_url = \"https:\/\/docs.rs\/quicksilver\/$version\/quicksilver\"/g" -i src/lib.rs

echo "--- Bumping version in Cargo.toml ---"
sed -e "s/^version = \".*\"/version=\"$version\"/g" -i Cargo.toml

