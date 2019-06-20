// TODO: File representing an entire pipeline

use crate::pipeline_stage::PipelineStage;
use crate::phv::Phv;

pub struct Pipeline {
   pipeline_stages : Vec<PipelineStage>,
}

impl Pipeline {
    
  pub fn new (t_pipeline_stages : &Vec <PipelineStage>) -> Self {
    Pipeline {pipeline_stages : t_pipeline_stages.clone() }
  }

  // Calls tick on current packet with all of the
  // stages in the pipeline
  pub fn tick (&self, packet : Phv) -> Phv {

    assert! (self.pipeline_stages.len() > 0);
    // If there's only 1 PipelineStage, just
    // call the first stage's tick

    let mut tmp_phv : Phv = packet.clone();
    // For every PipelineStage in the pipeline, call
    // tick function on current packet which
    // in turn calls all ALUs
    self.pipeline_stages
        .iter()
        .for_each ( |stage| {
          tmp_phv = stage.tick (tmp_phv.clone());
    });

    tmp_phv
  }
  
}

