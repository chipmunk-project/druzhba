Programmable switch pipeline simulator.

To execute, run the following:

    cargo build
    cargo run <comma-separated list of input fields> <comma-separated list of output fields> <ticks>

Example:

    cargo build
    cargo run arrival_time,dport,new_hop,next_hop,sport,last_time  arrival_time,dport,new_hop,next_hop,sport,last_time  100
