//mod phv;
//use crate::phv::Phv;

use crate::phv::Phv;
#[derive(Clone)]
pub struct PipelineStage {

}

impl PipelineStage {
  pub fn tick(&self, packet : Phv) -> Phv { 
    // TODO: Complete
    packet
  }
}
