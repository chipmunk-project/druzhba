use crate::phv_container::PhvContainer;

#[derive(Clone)]
pub struct OutputMux{
    pub input_phv_containers: Vec<i32>,
    pub index: i32
}

impl OutputMux{

    pub fn new(&self, input: Vec<i32>, i : i32) -> Self {
        OutputMux {input_phv_containers : input, index: i}
    }
    
    /*Add a Phv Container to the list of
     input PHV Containers supplied to the OutputMux*/

    pub fn add_phv_container(&mut self, phv_cont : i32) {
        self.input_phv_containers.push(phv_cont);
    }
    pub fn swap_input_phv_containers (&mut self, t_phv_containers : &Vec <i32>){
      self.input_phv_containers = t_phv_containers.clone();
    }
    
    /*  Use input index, to return a single PHV Container
     from a list of them */

    pub fn output(&self) -> PhvContainer<i32>{
        if self.input_phv_containers.len() == 0{
            panic!("Error: This output mux has no inputs");
        }
        PhvContainer {
            field_value : self.input_phv_containers[self.index as usize].clone()
        }
    }
}
