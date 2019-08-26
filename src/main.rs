extern crate rand;
extern crate druzhba;

mod prog_to_run;
mod test_files;

use druzhba::pipeline::Pipeline;
use druzhba::phv::Phv;
use druzhba::phv_container::PhvContainer;

use std::collections::HashMap;
use rand::Rng;
use std::env;
use std::fs;

// Opens hole configs file of hole variable assignments
// and initializes a HashMap from hole vaiables to
// i32s.
fn get_hole_cfgs (hole_cfgs_file : String) -> HashMap <String, i32> {

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
#[warn(unused_imports)]
fn main() {

  let args : Vec<String> = env::args().collect();
  assert!(args.len() == 3);

  // Parse returns a result so unwrap
  let ticks : i32 = 
    match args[1].parse::<i32>() {

      Ok  (ticks_arg) => ticks_arg,
      Err (_)         => panic!("Failure: Unable to unwrap ticks"),
    };
  let hole_cfgs_file : String = args[2].clone();
  let hole_cfgs : HashMap <String, i32> = get_hole_cfgs (hole_cfgs_file.clone());
  println!("{:?}", hole_cfgs);
  assert! (ticks >= 1);
  let mut pipeline : Pipeline = 
      prog_to_run::init_pipeline(hole_cfgs.clone());
  let num_containers : i32 = 2;
  // TODO: Determine how many state operands are needed.
  // Use 2 for now
  let num_state : i32 = 2;
  let num_stateful_alus : i32 = 1;
  // For every tick create a new packet with the 
  // specified input fields set to random values from
  // 0 to 100. Send packet through pipeline and 
  // retrieve resulting packet.
  let mut input_phvs : Vec <Phv <i32> > = Vec::new();
  let mut output_phvs : Vec <Phv <i32> > = Vec::new();
  // _t not used
  for _t in 0..ticks {
    
    let mut packet : Phv<i32> = Phv::new();
    // Initializes packet with all of the input fields
    // along with a random value
    (0..num_containers)
        // _s not used
        .for_each ( |_s| {
            packet.add_container_to_phv(PhvContainer {
                field_value :// 0,
                rand::thread_rng().gen_range(0,100),
            }); 
        });

//    packet.set_state (vec![0; num_state as usize]);
    let mut state : Vec <Vec <i32> > = Vec::new();
    for i in (0..num_stateful_alus){
      let mut tmp_state_vec : Vec<i32> = Vec::new();
      for j in (0..num_state) {
        tmp_state_vec.push(rand::thread_rng().gen_range(0,100));
      }
      state.push (tmp_state_vec);
    }
    packet.set_state(state);
    input_phvs.push (packet.clone());
    let new_packet : Phv<i32> = pipeline.tick (packet);

    if !new_packet.is_bubble() {
      output_phvs.push(new_packet.clone());
    }
  }
  println!("Input PHVs:\n");
  for phv in input_phvs {
    println!("{}", phv);
  }
  
  println!("\n\nOutput PHVs:\n ");
  for phv in output_phvs{
    println!("{}", phv);
  }
}
#[cfg(test)]
mod test_druzhba;

