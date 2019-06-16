// Represents a stage in the pipeline

use crate::phv::Phv;

#[derive(Clone)]

pub struct PipelineStage {
  // TODO: Add vector of ALUs
}

impl PipelineStage {

  // TODO: Create 1 parameter constructor
  // that takes in vector of ALUs

  pub fn tick(&self, packet : Phv) -> Phv { 
    // TODO: Complete. Should iterate through
    // ALUs in random order and each take in the
    // Phv
    packet
  }
}
