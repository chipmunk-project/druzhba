use crate::pipeline_stage::PipelineStage;
use crate::phv::Phv;
use std::mem;
use std::collections::HashMap;
use std::fmt;

 /*
    Banzai uses write and read latches to make sure that the output values of
    one stage don't become the input values of the next stage in a single tick (by writing an output into the read field). Im just using
    two hashmaps as a work-around.
*/

pub struct Pipeline {
   pipeline_stages : Vec<PipelineStage>,
   
   //Format : key = stage_number, value = PHV
   old_phvs: HashMap<usize, Phv<i32>>,
   new_phvs: HashMap<usize, Phv<i32>>,
   initial_phvs : HashMap<usize, Phv<i32>>
}

impl Pipeline {
    
  pub fn new () -> Self {
    let stages : Vec <PipelineStage> = Vec::new();
    Pipeline { pipeline_stages : stages, 
               old_phvs: HashMap::new(), 
               new_phvs : HashMap::new(),
               initial_phvs : HashMap::new()}
  }

  pub fn with_pipeline_stages (t_pipeline_stages : Vec <PipelineStage>) -> Self {
    let mut old_phv : HashMap<usize, Phv<i32>> = HashMap::new();
    let mut new_phv : HashMap<usize, Phv<i32>> = HashMap::new();
    for i in 0..t_pipeline_stages.len(){
      old_phv.insert(i as usize, Phv::new());
      new_phv.insert(i as usize, Phv::new());
    }
    Pipeline { pipeline_stages : t_pipeline_stages, 
               old_phvs: old_phv, 
               new_phvs: new_phv,
               initial_phvs : HashMap::new()}
  }

  pub fn len (&self) -> usize {
    self.pipeline_stages.len()
  }
  pub fn tick (&mut self, t_packet : Phv<i32>) -> (Phv<i32>, Phv<i32>) {
    if self.pipeline_stages.len() == 1{
      
      self.pipeline_stages[0].tick(t_packet.clone(), 
                                   t_packet)
    }
    else{
      let (first_initial_phv, first_result_phv) =                             self.pipeline_stages[0].tick(t_packet.clone(),
                                       t_packet.clone());

      self.new_phvs.insert(0, first_result_phv);
      self.initial_phvs.insert(0, first_initial_phv);

      for x in 1..self.pipeline_stages.len() - 1 {
        let (initial_phv, result_phv) = self.pipeline_stages[0].
            tick(self.initial_phvs[&(x-1)].clone(),
                 self.old_phvs[&(x-1)].clone());
        self.new_phvs.insert(x, result_phv);
        self.initial_phvs.insert(x, initial_phv);
      }

      let length : usize = self.pipeline_stages.len();

      let last_phvs : (Phv <i32>, Phv<i32>)  = self.pipeline_stages[length - 1]
        .tick(self.initial_phvs[&(length-2)].clone(),
              self.old_phvs[&(length - 2)].clone());
      mem::swap(&mut self.new_phvs, &mut self.old_phvs);
      last_phvs
    }
  }
}


//For Debugging Purposes
impl fmt::Display for Pipeline{

  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let mut s : String = String::from(""); 
      s.push_str( "Old Phvs: \n");
      for (key, value) in &self.old_phvs {
        s.push_str(&format!( "stage {} : \n{}\n", key, value));
      }

      s.push_str ("\nNew Phvs: \n");
      for (key_, value_) in &self.new_phvs {
        s.push_str( &format!("stage {} :  \n{}\n", key_, value_));
      }
    
      write!(f, "{}", s)
  }
}
