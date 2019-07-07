use crate::phv_container::PhvContainer;
use crate::phv::Phv;
use crate::alu::ALU;

/* Option being used for an output_value because their is no
  output_value until the mux actually executes */

#[derive(Clone)]
pub struct OutputMux{
    pub input_phv_containers: Vec<PhvContainer<i32>>,
    pub index: i32
}

impl OutputMux{

    pub fn new(&self, input: Vec<PhvContainer<i32>>, i : i32) -> Self {
        OutputMux {input_phv_containers : input, index: i}
    }
    
    /*Add a Phv Container to the list of input PHV Containers supplied to the OutputMux*/
    pub fn add_phv_container(&mut self, phv_cont : PhvContainer<i32>) {
        self.input_phv_containers.push(phv_cont);
    }
    
    /* input index will be supplied by Chipmunk. Use input index, 
    to return a single PHV Container from a list of them */
    pub fn output(&self) -> PhvContainer<i32>{
        self.input_phv_containers[self.index as usize].clone()
    }
}