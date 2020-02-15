[![Build status](https://ci.appveyor.com/api/projects/status/fkguas3fm87takto?svg=true)](https://ci.appveyor.com/project/anirudhSK/druzhba-j5ou7)


Programmable switch pipeline simulator.

Before the simulator can be executed, dgen needs to build
and run. After running dgen, a prog_to_run.rs Rust file
will be automatically generated in the Druzhba src directory 
which will allow the simulator to run.


Running dgen/dsim using the protocol independent switch architecture:

    python3 pisa_simulation_execute.py <program name> <stateful ALU> <stateless ALU> <pipeline depth> <pipieline width> <stateful ALUs per stage> <constant set> <hole configurations> <packets> <ticks> <optimization level>

Example:

    python3 pisa_simulation_execute.py simple example_alus/stateful_alus/raw.alu example_alus/stateless_alus/stateless_alu.alu 2 2 1 "0,1,2,3" hole_configurations/simple_raw_stateless_alu_2_2_hole_cfgs.txt 1 100 1

Running dsim using dRMT architecture (using P4):

    python3 p4_drmt_simulation_execute.py <P4 file> <num of packet fielsd> <ticks> <num of processors> <num of state variables> <path to drmt repository>

Example:

    python3 p4_drmt_simulation_execute.py p4_files/stateful.p4 2 10 4 1 /path/to/drmt
  
NOTE: p4-hlir (https://github.com/jafingerhut/p4-hlir) is needed, which converts .p4 file into DAG.
Must also have drmt repository (github.com/anirudhSK/drmt/) downloaded 


Tests will ensure the druhzba pipeline is outputting
the correct packets relative to the input packets
given to the pipeline. Run dgen first to get a 
prog_to_run file in src first. Otherwise the simulator
won't compile. 

To run these tests:

    ./build_dgen.sh && cargo test

To run benchmarks:

    ./build_dgen.sh && cargo bench

Similarly, the dgen tests ensure that the alu grammar
is being parsed correctly and that the ast is being
generated properly. 

To run these tests:

    cd dgen
    cargo test

Note: Rust nightly may need to be enabled before LALRPOP
can work

    rustup default nightly

