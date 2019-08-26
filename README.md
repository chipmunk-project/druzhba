Programmable switch pipeline simulator.

Before the simulator can be executed, dgen needs to build
and run. After running dgen, a prog_to_run.rs Rust file
will be automatically generated in the Druzhba src directory 
which will allow the simulator to run.

To execute dgen, run

    cd dgen
    cargo buld && cargo run <spec name> <stateful alu file> <stateless alu file> <pipeline depth> <pipeline width>

Example:

    cd dgen
    cargo build && cargo run simple ../example_alus/stateful_alus/raw.alu ../example_alus/stateless_alus/stateless_alu.alu 2 2


With dgen's generated prog_to_run.rs file constructed,
, run the following to execute Druzhba (note: as of this
pull request, Druzhba will use the hardcoded prog_to_run.rs 
file for now):

    cargo build && cargo run <ticks>

Example:

    cargo build && cargo run 100


Tests will ensure the druhzba pipeline is outputting
the correct packets relative to the input packets
given to the pipeline. 

To run these tests:

    cargo test

Similarly, the dgen tests ensure that the alu grammar
is being parsed correctly and that the ast is being
generated properly. 

To run these tests:

    cd dgen
    cargo test

Note: Rust nightly may need to be enabled before LALRPOP
can work

    rustup default nightly

