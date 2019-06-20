extern crate druzhba;
extern crate rand;

mod prog_to_run;

use druzhba::pipeline_stage::PipelineStage;
use druzhba::pipeline::Pipeline;
use druzhba::phv::Phv;
use druzhba::phv_container::PhvContainer;

use rand::Rng;
use std::env;
use std::process;

// Takes in a comma-separated &String and returns
// a vector of String 
fn string_from_csv (csv : &String) -> Vec <String>
{

  let str_vec : Vec <&str> = csv
      .split(",")
      .collect();
  // Converts Vec <&str> to Vec <string>
  str_vec
      .iter()
      .map ( |s| s.to_string())
      .collect()

}
fn main() {

  let args : Vec<String> = env::args().collect();
  assert!(args.len() == 4);
  let input_list : Vec <String> = string_from_csv (&args[1]);
  let output_list : Vec <String> = string_from_csv (&args[2]);

  // Parse returns a result so unwrap
  let ticks : i32 = 
    match args[3].parse::<i32>() {

      Ok  (ticks_arg) => ticks_arg,
      Err (_)         => panic!("Failure: Unable to unwrap ticks"),
    };
  assert! (ticks >= 1);
  let pipeline : Pipeline = prog_to_run::init_pipeline();

  // For every tick create a new packet with the 
  // specified input fields set to random values from
  // 0 to 100. Send packet through pipeline and 
  // retrieve resulting packet.
  for t in 0..ticks {
    let mut map : PhvContainer = PhvContainer::new();
    let mut packet : Phv = Phv::with_container (map);

    // Initializes packet with all of the input fields
    // along with a random value
    input_list
        .iter()
        .for_each ( |s| {
            packet[s] = rand::thread_rng().gen_range(0,100); 
        });

    println!("Input packet: {} ", packet);
    let new_packet : Phv = pipeline.tick (packet);

    if new_packet.is_bubble() {
      println! ("Output packet: {} ", new_packet);
    }
  }

}

