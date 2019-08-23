extern crate rand;

use crate::phv::Phv;
use crate::alu::ALU;
use crate::phv_container::PhvContainer;

use self::rand::{thread_rng, Rng};

pub struct PipelineStage {
   pub stateless_atoms : Vec<ALU>,
   pub stateful_atoms : Vec<ALU>,
}

impl PipelineStage {
  pub fn new () -> Self {
    PipelineStage { stateless_atoms : Vec::new(),
                    stateful_atoms : Vec::new(), }
  }
  pub fn with_atoms (stateless : Vec <ALU>, stateful : Vec<ALU>) -> Self{
    PipelineStage { stateless_atoms : stateless,
                    stateful_atoms : stateful, 
    }
  }

  // Iterates through all atoms stored and calls their 
  // underlying function on the incoming Phv in 
  // random order. Pass the mutated phv containers to their respective muxes.
  pub fn tick(&mut self, input_phv: Phv<i32>) -> Phv<i32>{ 

      if input_phv.is_bubble() {
        Phv::new()
      }
      else{
      
        let mut output_phv : Phv<i32> = 
            Phv { bubble : false, 
                  packets: Vec::new() };
        thread_rng().shuffle(&mut self.stateful_atoms);
        thread_rng().shuffle(&mut self.stateless_atoms);

        let mut old_state : Vec <i32> = Vec::new();
        // Need old state variables first to put them
        // into output muxes later
        for atom in self.stateful_atoms.iter_mut () {
          atom.send_packets_to_input_muxes(input_phv.clone());
          let mut packet_fields : Vec<PhvContainer<i32>> = atom.input_mux_output();
          for elem in atom.run (&mut packet_fields){
            old_state.push (elem);
          }
        }
        // Gets return values from the ALUs and inserts
        // them into output muxes along with old state vars
        for atom in self.stateless_atoms.iter_mut() {
        
          //PHV is passed to it's corresponding input mux, and
          //a single container is outputted. Container is put
          //into a vector and passed to atom
          atom.send_packets_to_input_muxes(input_phv.clone());
          let packet_fields : Vec<PhvContainer<i32>> = 
              atom.input_mux_output();
          //After being passed to atom, value is sent to an
          //output mux and put into a PHV

          let result : i32 =  atom.run(&packet_fields)[0];
          // State variables and returned value from stateless ALU
          let mut output_mux_fields : Vec <i32> = old_state.clone();

          output_mux_fields.push (result);

          atom.send_packets_to_output_mux(&output_mux_fields);
          output_phv.add_container_to_phv(atom.output_mux.output());
        }
        output_phv
      }
    }
  }
