
use crate::phv_container::PhvContainer;
use crate::phv::Phv;
use crate::output_mux::OutputMux;
use crate::input_mux::InputMux;
use std::collections::HashMap;

//Use a PHV Container as an object containing all states - *just for now*
//return the previous states, unmodified or not


#[derive(Clone)]
pub struct ALU{
    
    /*Packet containers from multiple muxs' are passed to an ALU, so should be a struct field and organized
    as a vector of containers compared to a PHV*/
    
    pub sequential_function :
        fn (&mut HashMap<String, i32>,
            &mut Vec<PhvContainer<i32>>
            ),
        
    //Change this to a state-variable
    pub state_variables : HashMap<String, i32>,
    pub input_mux : InputMux,
    pub output_mux : OutputMux
}

impl ALU {
       
    pub fn new (function : fn ( &mut HashMap<String, i32>, 
                                &mut Vec<PhvContainer<i32>>),
            state_vars : HashMap<String, i32>,
            imux: InputMux,
            omux : OutputMux) 
            -> ALU {

      ALU { sequential_function : function,
            state_variables: state_vars,
            input_mux: imux,
            output_mux : omux }
    }

    // Receives mutable reference to Phv and calls underlying 
    // function, sequential_function using state_scalar and 
    // state_array. Mutates Phv in place with appropriate 
    // packet values. Once function is run, phv value should
    // be passed to the output mux. 

    pub fn run (&mut self, packet_fields: &mut Vec<PhvContainer<i32>>) {

      (self.sequential_function) 
          (&mut self.state_variables,
           packet_fields
           );
    }

    pub fn input_mux_output(&self) -> PhvContainer<i32>{
        self.input_mux.output()
    }
    pub fn send_packets_to_input_mux(&mut self, values: Phv<i32>) {
        self.input_mux.input_phv = values;   
    }
    pub fn send_packets_to_output_mux(&mut self, mut values: Vec<PhvContainer<i32>>) {
        self.output_mux.input_phv_containers.append(&mut values);   
    }  
}


