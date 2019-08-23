Programmable switch pipeline simulator.

To execute, run the following:

    cargo build
    cargo run <ticks>

Example:

    cargo build
    cargo run 100

Tests will ensure that alu grammar is being parsed
correctly and that the druhzba pipeline is outputting
the correct packets. The last test in test_grammar.rs
(test_output) will generate src/test_prog_to_run.rs
which contains the generated Rust code. 

To run these tests:

    cargo test

Note: Rust nightly may need to be enabled before LALRPOP
can work

    rustup default nightly

