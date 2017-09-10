#!/bin/bash

cd /var/lib/money_map/statement_gen
echo $(pwd)
cargo build
cargo run
