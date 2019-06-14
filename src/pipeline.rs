// TODO: File representing an entire pipeline

use crate::pipeline_stage::PipelineStage;
use crate::phv::Phv;

pub struct Pipeline {
   stages : Vec<PipelineStage>,
}
impl Pipeline {
    
  pub fn new (t_stages : &Vec <PipelineStage>) -> Self {
    Pipeline {stages : t_stages.clone() }
  }
  // Calls tick on current packet with all of the
  // stages in the pipeline
  pub fn tick (&self, packet : Phv) -> Phv {
    assert! (self.stages.len() >0);
    if self.stages.len()==1 {
      self.stages[0].tick (packet)
    }
    else {

      let mut tmp_phv : Phv = packet.clone();

      for i in 0..self.stages.len()-1{
        if i==0 {
          tmp_phv = self.stages[i].tick (packet.clone());
        }
        else {
          tmp_phv = self.stages[i].tick(tmp_phv.clone());
        }
      }
      self.stages[self.stages.len()-1]
          .tick(tmp_phv
          .clone())
    }
  }
  
}

