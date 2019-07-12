extern crate rand;

use crate::phv::Phv;
use crate::alu::ALU;
use crate::phv_container::PhvContainer;

use std::collections::HashMap;
use self::rand::{thread_rng, Rng};


#[derive(Clone)]
pub struct PipelineStage {
   pub atoms : Vec<ALU>,
}

impl PipelineStage {
  pub fn new () -> Self {
    let vec_of_atoms : Vec <ALU> = Vec::new();
    PipelineStage { atoms : vec_of_atoms }
  }
  pub fn with_atoms (vec_of_atoms : Vec <ALU> ) -> Self{
    PipelineStage { atoms : vec_of_atoms }
  }
  pub fn add_atom (&mut self, t_alu : ALU) {
    self.atoms.push (t_alu)
  }

  // Iterates through all atoms stored and calls their 
  // underlying function on the incoming Phv in 
  // random order. Pass the mutated phv containers to their respective muxes.
  pub fn tick(&self, input_phv: Phv<i32>) -> Phv<i32>{ 

      if (input_phv.is_bubble()) {
          Phv::new()
      }
      else{
      
      let mut output_phv : Phv<i32> = Phv {bubble : false, packets: Vec::new() };
      let mut tmp_atoms : Vec <ALU> = self.atoms.clone();
      thread_rng().shuffle(&mut tmp_atoms);

      for atom in tmp_atoms.iter_mut() {
        
        //PHV is passed to it's corresponding input mux, and
        //a single container is outputted. Container is put
        //into a vector and passed to atom
        atom.send_packets_to_input_mux(input_phv.clone());
        let output_of_input_mux : PhvContainer<i32> = atom.input_mux_output();
        let mut packet_fields : Vec<PhvContainer<i32>> = vec![output_of_input_mux];
        
        //After being passed to atom, value is sent to an
        //output mux and put into a PHV
        atom.run(&mut packet_fields);
        atom.send_packets_to_output_mux(packet_fields);
        output_phv.add_container_to_phv(atom.output_mux.output());
      }

      output_phv

      }
  
    }
  }
