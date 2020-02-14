extern crate rand;
extern crate druzhba;

mod prog_to_run;
mod tests;
mod match_action_ops;
use druzhba::pipeline::Pipeline;
use druzhba::phv::Phv;
use druzhba::phv_container::PhvContainer;
use druzhba::processor::Processor;
use druzhba::drmt_processor::dRMTProcessor;
use druzhba::scheduler::Scheduler;
use druzhba::packet::Packet;
use druzhba::stateful_memory::{StatefulMemory,StatefulMemories};
use rand::Rng;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

// Opens hole configs file of hole variable assignments
// and initializes a HashMap from hole vaiables to
// i32s.
fn extract_hole_cfgs (hole_cfgs_file : String) -> HashMap <String, i32> {
    
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

fn generate_random_phv (num_packet_fields : i32) -> Phv <i32> {
    let mut phv : Phv<i32> = Phv::new();
    
    (0..num_packet_fields)
         // _s not used
        .for_each ( |_s| {
         phv.add_container_to_phv(PhvContainer {
             field_value :rand::thread_rng().gen_range(0,100),
         }); 
      });
    phv
 
}
// Initiate RMT pipeline simulation using prog_to_run.rs
// provided by dgen
fn execute_rmt (args : Vec<String>)
{
  // Parse returns a result so unwrap
  let num_stateful_alus = prog_to_run::num_stateful_alus();
  let num_state_values = prog_to_run::num_state_variables();
  assert! (num_stateful_alus>=1);
  let num_packet_fields : i32 = 
      match args.len() == 5 {
        true =>  match args[3].parse::<i32>() {

          Ok  (t_pkts) => t_pkts,
          Err (_)         => panic!("Failure: Unable to unwrap ticks"),
        },
        false => match args[2].parse::<i32>() {

          Ok  (t_pkts) => t_pkts,
          Err (_)         => panic!("Failure: Unable to unwrap ticks"),
        },
    };

  assert!(num_packet_fields <= prog_to_run::pipeline_width());
  let ticks : i32 = 
      match args.len() == 5 {
        true =>  match args[4].parse::<i32>() {

          Ok  (t_ticks) => t_ticks,
          Err (_)         => panic!("Failure: Unable to unwrap ticks"),
        },
        false => match args[3].parse::<i32>() {

          Ok  (t_ticks) => t_ticks,
          Err (_)         => panic!("Failure: Unable to unwrap ticks"),
        },
    };
  assert! (ticks >= 1);
     
  let mut pipeline : Pipeline = 
      match args.len() == 5 {
        true  => 
            prog_to_run::init_pipeline(extract_hole_cfgs(args[1].clone())),
        // TODO: REmove hashmap argument when possible
        false => {
            println!("Args len not 5");
            prog_to_run::init_pipeline(HashMap::new())
        },
      };

  println!("Initializing phvs");
  // For every tick create a new packet with the 
  // specified input fields set to random values from
  // 0 to 100. Send packet through pipeline and 
  // retrieve resulting packet.
  let mut input_phvs : Vec <Phv <i32> > = Vec::new();
  let mut output_phvs : Vec <Phv <i32> > = Vec::new();
  // _t not used
  for _t in 0..ticks {
    let mut phv : Phv<i32> = generate_random_phv (num_packet_fields);
    (num_packet_fields..prog_to_run::pipeline_width())
        .for_each( |_s| { 
            phv.add_container_to_phv (PhvContainer{
                field_value : 0,
            });
        });

    let mut state : Vec <Vec <i32> > = Vec::new();
    // _i not used
    for _i in 0..num_stateful_alus{
      let mut tmp_state_vec : Vec<i32> = Vec::new();
      // _j not used
      for _j in 0..num_state_values {
        tmp_state_vec.push(0);
           
      }
      state.push (tmp_state_vec);
    }
    phv.set_state(state);
    let updated_input_output_phvs: (Phv<i32>, Phv<i32>) = 
        pipeline.tick (phv);

    let updated_input_phv = updated_input_output_phvs.0;
    let output_phv = updated_input_output_phvs.1;

    if !output_phv.is_bubble() {

      input_phvs.push (updated_input_phv.clone());
      output_phvs.push(output_phv.clone());
    }
  }
  for i in 0..output_phvs.len(){
    println!("Input: {}", input_phvs[i]);
    println!("Result: {}\n", output_phvs[i]);
  }
}

// Simulates using dRMT architecture
fn execute_drmt (args : Vec <String>)
{
    let input_file : &str = &args[2];
    let num_packet_fields : i32 = 
      match args[3].parse::<i32>() {

        Ok  (t_pkts)    => t_pkts,
        Err (_)         => panic!("Failure: Unable to unwrap num_packet_fields"),
      };

    let ticks : i32 = 
      match args[4].parse::<i32>() {

        Ok  (t_ticks) => t_ticks,
        Err (_)       => panic!("Failure: Unable to unwrap ticks"),
      };
    let num_processors : i32 = 
      match args[5].parse::<i32>() {

        Ok  (t_num_processors)    => t_num_processors,
        Err (_)                   => panic!("Failure: Unable to unwrap num_processors"),
      };

    let num_state_vars : i32 = 
      match args[6].parse::<i32>() {

        Ok  (t_num_state_vars)   => t_num_state_vars,
        Err (_)                   => panic!("Failure: Unable to unwrap num_state_vars"),
      };

    assert! (ticks >= 1);
    let mut processors : Vec<Processor> = Vec::new();
    // _i not used
    for _i in 0..num_processors {
        processors.push(Processor {
          riscv_file : input_file.to_string(),
          phvs : Vec::new(),
          state : vec![0; num_state_vars as usize]}); 
    }
    for t in 0..ticks {
        let mut phv : Phv <i32> = 
            generate_random_phv(num_packet_fields);
        println!("Cycle : {}, processor: {}", t, t % num_processors);
        println!("Input: {}", phv);
        processors[(t % num_processors) as usize]
                    .add_phv(phv);

        processors[(t % num_processors) as usize]
                    .execute_program();
    }
}

fn execute_p4_drmt (args : Vec <String>) 
{
    let schedule =  match_action_ops::generate_schedule();
    println!("Schedule: {:?}", schedule);
    let p4_input_file : &str = &args[2];
    let num_packet_fields : i32 = 
    match args[3].parse::<i32>() {

        Ok  (t_pkts)    => t_pkts,
        Err (_)         => panic!("Failure: Unable to unwrap num_packet_fields"),
      };

    let ticks : i32 = 
      match args[4].parse::<i32>() {

        Ok  (t_ticks) => t_ticks,
        Err (_)       => panic!("Failure: Unable to unwrap ticks"),
      };
    let num_processors : i32 = 
      match args[5].parse::<i32>() {

        Ok  (t_num_processors)    => t_num_processors,
        Err (_)                   => panic!("Failure: Unable to unwrap num_processors"),
      };

    let num_state_vars : i32 = 
      match args[6].parse::<i32>() {

        Ok  (t_num_state_vars)   => t_num_state_vars,
        Err (_)                   => panic!("Failure: Unable to unwrap num_state_vars"),
      };
//    let path_to_drmt : &str = &args[7];
    
    assert! (ticks >= 1);
 //   let scheduler : Scheduler = Scheduler { input_file : p4_input_file.to_string() };
//    let mut cycles_to_matches_and_actions : HashMap <i32, Vec<String> > = 
//      scheduler.exec_drmt_scheduler(path_to_drmt);
/*
    let mut max : i32 = 0;
     // Get max tick
    for k in cycles_to_matches_and_actions.keys(){
      if k > &max {
        max = *k;
      }
    }
    // TODO: Allow more hw and latency configurations apart
    // from just large_hw.py and drmt_latencies.py
    let hw_file : &str = "drmt_specs/large_hw.py";
    let latencies_file : &str = "drmt_specs/drmt_latencies.py";
    if Path::new(latencies_file).exists(){
      let contents : String = fs::read_to_string(&latencies_file)
        .expect("Something went wrong with reading latencies file");
      let latencies_vec : Vec<String> = contents
                                        .split ("\n")
                                        .map (|s| s.to_string())
                                        .collect();
      for line in latencies_vec.iter() {
        let assignment_line : Vec <String> = line
                                             .split("=")
                                             .map(|s| s.to_string())
                                             .collect();
        if assignment_line[0].trim() == "dA" {
          max += assignment_line[1].trim()                                                             .parse::<i32>()  
                                   .unwrap();
 
        }
      }

    }

    println!("Schedule takes {} ticks", max);
    let mut processors : Vec <dRMTProcessor> = Vec::new();
    for i in 0..num_processors {
      let processor : dRMTProcessor = dRMTProcessor {
        processor_id : i,
        schedule : cycles_to_matches_and_actions.clone(),
        phvs_and_initial_tick : Vec::new(),
        phv_output_strings : HashMap::new(),
        state : Vec::new(),
        current_tick : -1,
        max_tick : max,
      };
      processors.push(processor);
    }
      
    println!("Beginning simulation");
    for t in 0..ticks {
      let mut phv : Phv<i32> = 
        generate_random_phv (num_packet_fields);
      processors [(t % num_processors) as usize].add_phv (phv, t);
      for i in 0..processors.len() {
        processors [i as usize].tick();
      }
    }
*/
}

#[warn(unused_imports)]
fn main() {

  let args : Vec<String> = env::args().collect();
  let arch : &str = &args[1];
  println!("Args: {:?}", args);
  match arch {
    "dRMT" => {
                assert!(args.len() == 7);
                execute_drmt(args);
    },
    "drmt" => {
                assert!(args.len() == 7);
                execute_drmt(args);
    },
    "RMT"  => {
                assert!(args.len() == 4 || args.len() == 5);
                execute_rmt(args);
    },
    "rmt"  => {
                assert!(args.len() == 4 || args.len() == 5);
                execute_rmt(args);
    },
    "drmt_p4" => {
                assert!(args.len() == 7);
                execute_p4_drmt(args);

    },
    _      => panic!("Incorrect architecture. Only rmt and drmt are supported"),
  };

}
#[cfg(test)]
mod test_druzhba;
mod test_with_chipmunk;
