Programmable switch pipeline simulator.

Before the simulator can be executed, dgen needs to build
and run. After running dgen, a prog_to_run.rs Rust file
will be automatically generated in the Druzhba src directory 
which will allow the simulator to run.

To execute dgen, run

    cd dgen
    cargo build && cargo run <spec name> <stateful alu file> <stateless alu file> <pipeline depth> <pipeline width> <number of stateful ALUs>

Example:

    cd dgen
    cargo build && cargo run simple ../example_alus/stateful_alus/raw.alu ../example_alus/stateless_alus/stateless_alu.alu 2 2 1

With dgen's generated prog_to_run.rs file constructed,
, run the following to execute Druzhba:

    cd ..
    cargo build && cargo run <hole configuration file> <number of containers per phv> <number of stateful ALUs> <ticks>

Example:

    cd ..
    cargo build && cargo run hole_configurations/simple_raw_stateless_alu_2_2_hole_cfgs.txt 2 100

For quick execution using the build.sh script, use:

    ./build.sh <spec name> <stateful ALU file> <stateless ALU file> <pipeline width> <pipeline depth> <hole configuration file> <containers per Phv> <ticks>

Example:

    ./build.sh simple example_alus/stateful_alus/raw.alu example_alus/stateless_alus/stateless_alu.alu 2 2 1 hole_configurations/simple_raw_stateless_alu_2_2_hole_cfgs.txt 2 5

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

