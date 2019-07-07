// extern crate druzhba;
extern crate rand;

mod program_to_run_akv;

extern crate druzhba;

use druzhba::pipeline::Pipeline;
use druzhba::phv::Phv;
use druzhba::phv_container::PhvContainer;
use druzhba::pipeline_stage::PipelineStage;
// use druzhba::program_to_run_akv;

use std::collections::HashMap;
use rand::Rng;
use std::env;

#[warn(unused_imports)]
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
  let mut pipeline : Pipeline = program_to_run_akv::init_pipeline();

  // For every tick create a new packet with the 
  // specified input fields set to random values from
  // 0 to 100. Send packet through pipeline and 
  // retrieve resulting packet.
  for _t in 0..ticks {
    
    let mut packet : Phv<i32> = Phv::new();
    // Initializes packet with all of the input fields
    // along with a random value
    input_list
        .iter()
        .for_each ( |s| {
            packet.add_container_to_phv(PhvContainer {field_value : rand::thread_rng().gen_range(0,100)});            
        });

    println!("Tick {}", _t);
    println!("Input packet: {} ", packet);
    // println!("Is bubble: {}", packet.is_bubble());
    let new_packet : Phv<i32> = pipeline.tick (packet);
    // println!("Is bubble: {}", new_packet.is_bubble());
    // println!("Value of first container in packet {}", new_packet.packets[0].field_value);

    if !new_packet.is_bubble() {
      println! ("Output packet: {} ", new_packet);
    }
  }
}

