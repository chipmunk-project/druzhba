  
use druzhba::phv_container::PhvContainer;
use druzhba::pipeline_stage::PipelineStage;
use druzhba::pipeline::Pipeline;
use druzhba::alu::ALU;
use druzhba::input_mux::InputMux;
use druzhba::output_mux::OutputMux;use druzhba::phv::Phv;
use std::collections::HashMap;
use rand::Rng;
use std::fs;
use std::collections::HashMap;

/* Test helper functions */
fn init_input_phvs (ticks : i32,
                   num_containers : i32,
                   num_state_vars : i32,
                   num_stateful_alus : i32) -> Vec <Phv <i32> >
{
  let mut input_phvs : Vec <Phv <i32> > = Vec::new();
    // Initializes packet with all of the input fields
    // along with a random value
  for _i in 0..ticks {
    let mut packet : Phv<i32> = Phv::new();
    for _j in (0..num_containers){
        packet.add_container_to_phv(PhvContainer {
            field_value : rand::thread_rng().gen_range(0,100),
      }); 
    }
    let mut state_vec : Vec <Vec <i32>>  = Vec::new();
    for _i in 0..num_stateful_alus {
      let mut tmp_state_vec : Vec <i32> = Vec::new();
      for _j in 0..num_state_vars {
        tmp_state_vec.push (rand::thread_rng().gen_range(0,100));
      }
      state_vec.push (tmp_state_vec);
    }
    packet.set_state (state_vec);
    input_phvs.push (packet.clone());
  }
  input_phvs

}
// Initializes hole config HashMap and initializes pipeline.
// Runs input phvs through pipeline and returns the resulting
// output phvs
fn extract_hole_cfgs (hole_cfgs_file : String) -> HashMap <String, i32 > {
  let mut hole_cfgs_map : HashMap <String, i32> = HashMap::new();
  let hole_cfgs_file_contents : String = fs::read_to_string(hole_cfgs_file).expect ("Error: Hole configs file could not be found");
  let hole_cfgs_file_vec : Vec <String> = hole_cfgs_file_contents
                                          .split ("\n")
                                          .map (|s| s.to_string())
                                          .collect();

  for hole_var in hole_cfgs_file_vec {
      let hole_entry : Vec <&str> = hole_var
                                    .split("=")
                                    .map(|s| s.trim())
                                    .collect();
      if hole_entry.len() < 2 {
        continue;
      }
      hole_cfgs_map.insert (hole_entry[0].to_string(), 
                            hole_entry[1].to_string().parse::<i32>()
                                                     .expect ("Error: hole value set to non-integer value"));
  }
  hole_cfgs_map
}

fn run_pipeline (input_phvs : Vec <Phv <i32> >,
                 mut pipeline : Pipeline,
                 ticks : i32) -> Vec <Phv <i32 > > {
  let mut output_phvs : Vec <Phv <i32> > = Vec::new();
  for t in 0..ticks {
    let new_packet : Phv<i32> = 
        pipeline.tick (input_phvs[t as usize].clone());
    if !new_packet.is_bubble() {
      output_phvs.push(new_packet.clone());
    }
  }
  output_phvs
}


// Maintains the same assertions as the specification
// for the simple.sk spec in Chipmunk
fn test_simple (input_phvs : Vec <Phv <i32> >,
                output_phvs : Vec <Phv <i32> >) {

  for i in 0..output_phvs.len() {
    assert!(output_phvs[i][0].get_value() ==
            input_phvs[i].get_state()[0][0] + 1);

  }
}
fn test_marple_new_flow (input_phvs : Vec <Phv <i32> >,
                         output_phvs : Vec <Phv <i32> >) {

  for i in 0..output_phvs.len() {
    if input_phvs[i].get_state()[0][0] == 0 {
      assert! (output_phvs[i][0].get_value()==1);
      assert! (output_phvs[i].get_state()[0][0]==1);
    }
  }
}
fn test_blue_increase (input_phvs : Vec <Phv <i32> >,
                       output_phvs : Vec <Phv <i32> >) {

  for i in 0..output_phvs.len() {
    assert!(output_phvs[i][1].get_value() == input_phvs[i][0].get_value() - 1);
    if output_phvs[i][1].get_value() > input_phvs[i].get_state()[1][0]{
      assert!(output_phvs[i].get_state()[0][0] == input_phvs[i].get_state()[0][0]+1);
      assert!(output_phvs[i].get_state()[1][0] == input_phvs[i][0].get_value());
    }
  }
}


