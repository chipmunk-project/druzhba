fn main() {
  // List of all stages
  let pipeline_stages = Vec::<PipelineStage>();
  // Outer simulation loop
  for tick in 0..10000 {
    println!("Tick number {}", tick);
    for stage in stages {
      stage.tick(tick);
    }
  }
}
