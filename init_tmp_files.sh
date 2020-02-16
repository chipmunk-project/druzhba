#!/bin/bash

cd dgen && cargo build
cp target/debug/dgen dgen_bin
chmod +x dgen_bin
./dgen_bin pisa simple ../example_alus/stateful_alus/raw.alu ../example_alus/stateless_alus/stateless_alu.alu 2 2 1 "0,1,2,3" ../src/prog_to_run.rs

./dgen_bin dRMT ../p4_files/stateful.p4 /random/incorrect/path/ ../src/match_action_ops.rs
cd ..

