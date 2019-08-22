extern crate rand;

mod prog_to_run;

extern crate druzhba;

use druzhba::pipeline::Pipeline;
use druzhba::phv::Phv;
use druzhba::phv_container::PhvContainer;

use rand::Rng;
use std::env;

#[warn(unused_imports)]
fn main() {

  let args : Vec<String> = env::args().collect();
  assert!(args.len() == 2);

  // Parse returns a result so unwrap
  let ticks : i32 = 
    match args[1].parse::<i32>() {

      Ok  (ticks_arg) => ticks_arg,
      Err (_)         => panic!("Failure: Unable to unwrap ticks"),
    };
  assert! (ticks >= 1);
  let mut pipeline : Pipeline = prog_to_run::init_pipeline();
  let num_containers : i32 = 1;
  // For every tick create a new packet with the 
  // specified input fields set to random values from
  // 0 to 100. Send packet through pipeline and 
  // retrieve resulting packet.
  // _t not used
  for _t in 0..ticks {
    
    let mut packet : Phv<i32> = Phv::new();
    // Initializes packet with all of the input fields
    // along with a random value
    (0..num_containers)
        // _s not used
        .for_each ( |_s| {
            packet.add_container_to_phv(PhvContainer {
                field_value : rand::thread_rng().gen_range(0,100),
            }); 
        });

    println!("Tick Number: {} ", _t);
    println! ("Input packet: {} ", packet);
    let new_packet : Phv<i32> = pipeline.tick (packet);
    if !new_packet.is_bubble() {
      println! ("Output packet: {}\n\n", new_packet);
    }
  }
}
