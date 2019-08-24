Programmable switch pipeline simulator.

To execute, run the following:

    cargo build && cargo run <ticks>

Example:

    cargo build && cargo run 100

To execute dgen, run

    cd dgen
    cargo buld && cargo run <spec name> <stateful alu file> <stateless alu file> <pipeline depth> <pipeline width>

Example:

    cd dgen
    cargo build && cargo run simple ../example_alus/stateful_alus/raw.alu ../example_alus/stateless_alus/stateless_alu.alu 2 2

Tests will ensure that alu grammar is being parsed
correctly and that the druhzba pipeline is outputting
the correct packets. The last test in test_grammar.rs
(test_output) will generate src/test_prog_to_run.rs
which contains the generated Rust code. 

To run these tests:

    cargo test

Similarly, for dgen:

    cd dgen
    cargo test

Note: Rust nightly may need to be enabled before LALRPOP
can work

    rustup default nightly

