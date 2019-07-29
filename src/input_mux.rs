use crate::phv_container::PhvContainer;
use crate::phv::Phv;
use crate::alu::ALU;

/* Option being used for an output_value because their is no
  output_value until the mux actually executes */

#[derive(Clone)]
pub struct InputMux{
    pub input_phv: Phv<i32>,
    pub index: i32,
}

impl InputMux{

    pub fn new(&self, input: Phv<i32>, i : i32) -> Self {
        InputMux {input_phv : input, index : i}
    
    }
    pub fn output(&self) -> PhvContainer<i32> {
        self.input_phv[self.index].clone()   
    }
}
