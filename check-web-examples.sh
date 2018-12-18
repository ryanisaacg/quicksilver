#!/bin/sh
set -e

for example in $(ls examples);
do
    name=$(echo $example | cut -f 1 -d '.');
    cargo web check --example $name;
done
