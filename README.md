Programmable switch pipeline simulator.

To execute, run the following:

    cargo build
    cargo run <comma-separated list of input fields> <comma-separated list of output fields> <ticks>

Example:

    cargo build
    cargo run arrival_time,dport,new_hop,next_hop,sport  arrival_time,dport,new_hop,next_hop,sport  100
