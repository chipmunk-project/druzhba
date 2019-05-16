extern crate druzhba;

use druzhba::pipeline_stage;
use druzhba::pipeline;
fn main() {
  // List of all stages
  let pipeline_stages = vec![pipeline_stage::PipelineStage{};10];
  // Outer simulation loop
  for tick in 0..100 {
    println!("Tick number {}", tick);
    for stage in &pipeline_stages {
      stage.tick(tick);
    }
  }
}
