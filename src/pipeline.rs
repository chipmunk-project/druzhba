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
    // If there's only 1 PipelineStage, just
    // call the first stage's tick
    if self.stages.len()==1 {
      self.stages[0].tick (packet)
    }
    else {

      let mut tmp_phv : Phv = packet.clone();
      // For every PipelineStage in the pipeline, call
      // tick function on current packet which
      // in turn calls all ALUs
      //
      // TODO: If possible, see if this can be done using
      // a more functional approach
      for i in 0..self.stages.len()-1{
        if i==0 {
          tmp_phv = self.stages[i].tick (packet.clone());
        }
        else {
          tmp_phv = self.stages[i].tick(tmp_phv.clone());
        }
      }
      // Return packet of the last stage
      self.stages[self.stages.len()-1]
          .tick(tmp_phv
          .clone())
    }
  }
  
}

