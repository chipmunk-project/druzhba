#!/bin/bash

cd dgen && cargo build 
cargo run simple ../example_alus/stateful_alus/raw.alu ../example_alus/stateless_alus/stateless_alu.alu 2 2 1 "0,1,2,3" ../src/prog_to_run.rs
cd ..
