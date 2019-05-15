// TODO: File representing a single pipeline stage
#[derive(Clone)]
pub struct PipelineStage {

}

impl PipelineStage {
  pub fn tick(&self, tick : u8) { println!("Current tick is {}", tick)}
}