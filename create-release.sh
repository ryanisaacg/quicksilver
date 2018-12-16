#!/bin/sh
set -e
cargo test
echo '--- Remove the old build artifacts ---'
rm -f docs/*.examples.js docs/*.wasm docs/*.png docs/*.ttf docs/*.atlas docs/*.ogg
for example in examples/*
do
    example=$(basename $example .rs)
    echo "---- Building example: $example ----"
    cargo web build --release --example $example
    echo "--- Copying build artifacts: $example ----"
    cp target/wasm32-unknown-unknown/release/examples/$example.wasm docs/$example.wasm
    cp target/wasm32-unknown-unknown/release/examples/$example.js docs/$example.example.js
done

echo "--- Copying assets to the web directory ---"
cp static/* docs/
read -p "New version string: " version

echo "--- Bumping version of the html_url_root ---"
sed -e "s/html_root_url = \".*\"/html_root_url = \"https:\/\/docs.rs\/quicksilver\/$version\/quicksilver\"/g" -i.bak src/lib.rs

echo "--- Bumping version in Cargo.toml ---"
sed -e "s/^version = \".*\"/version = \"$version\"/g" -i.bak Cargo.toml

echo "--- Bumping version in the changelog ---"
sed -e "s/## In-development/## In-development\n\n## $version/" -i.bak CHANGES.md

echo "--- Cleaning up the backups ---"
rm -f *.bak src/lib.rs.bak
