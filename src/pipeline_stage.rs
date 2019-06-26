// Represents a stage in the pipeline

extern crate rand;

use crate::phv::Phv;
use crate::alu::ALU;
use crate::phv_container::PhvContainer;

use std::collections::HashMap;
use self::rand::{thread_rng, Rng};
#[derive(Clone)]

pub struct PipelineStage {
   atoms : Vec<ALU>,
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
  // random order. Returns the Phv of the summation
  // of all atom runs on the Phv.
  pub fn tick(&self, t_packet : Phv) -> Phv { 

    if t_packet.is_bubble(){
      t_packet
    }
    
    else {

      let map : HashMap <String, i32> = HashMap::new();
      let container : PhvContainer <i32> = PhvContainer::with_map(map);
      let mut ret = Phv::with_container(container);
      let mut tmp_atoms : Vec <ALU> = self.atoms.clone();
      // TODO: Currently shuffling tmp_atoms. Consider
      // adding self.atoms. To do so, we would also need
      // to make tick functions in this file and in 
      // pipeline.rs take mutable reference to self
      thread_rng().shuffle(&mut tmp_atoms);

      // Goes through atoms in random order and calls 
      // each atom on incoming packet. Accumulate them
      // all together in ret
      for atom in tmp_atoms.iter_mut() {

        let mut current_phv = t_packet.clone();
        atom.run (&mut current_phv);
        ret += current_phv;

      }
      ret
    }
  }
}
