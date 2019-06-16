extern crate druzhba;

use druzhba::pipeline_stage::PipelineStage;
use druzhba::pipeline::Pipeline;
use druzhba::phv::Phv;
use druzhba::phv_container::PhvContainer;

fn main() {
  // List of all stages
  let pipeline_stages = vec![PipelineStage{};5];
  
  let pipeline : Pipeline = Pipeline::new(&pipeline_stages); 

  // Iterate from 0 to 9 and test that Phv 
  // and PhvContainer can be initialized. 
  // Pumps each Phv through pipeline. 
  //
  // TODO: Update as pipeline_stage.rs and 
  // alu.rs become completed
  for i in 0..10{
    let mut map : PhvContainer = PhvContainer::new();
    map["field0"]= 0;
    map["field1"]= 1;
    let packet : Phv = Phv::with_container(map);
    let new_packet : Phv = pipeline.tick(packet);
    println!("Packet {} : {}", i , new_packet);
  }
}

