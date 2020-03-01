extern crate rand;
extern crate druzhba;

mod prog_to_run;
mod tests;
mod match_action_ops;
mod drmt_utils;
use druzhba::pipeline::Pipeline;
use druzhba::phv::Phv;
use druzhba::phv_container::PhvContainer;
use druzhba::drmt_processor::dRMTProcessor;
use druzhba::packet::Packet;
use druzhba::stateful_memory::{StatefulMemory,StatefulMemories};
use druzhba::match_action::MatchAction;
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
fn generate_random_packet (header_data : HashMap <String, HashMap <String, i32>>,
                           types_to_instances : HashMap <String, String>) -> Packet {
  let mut packet_data_fields : HashMap <String, HashMap <String, i32>> = HashMap::new();
  let metadata_fields : Vec <String> = 
                                vec!["ingress_port".to_string(),
                                     "packet_length".to_string(),
                                     "egress_spec".to_string(),
                                     "egress_port".to_string(),
                                     "egress_instance".to_string(),
                                     "instance_type".to_string(),
                                     "parser_status".to_string(),
                                     "parser_error_location".to_string()];
                                            
  // Populate packet with random header_types
  for (ht, pf_map) in header_data.iter() {

    let chance_to_add : i32 = rand::thread_rng().gen_range(0,4);

    let instances_name : String = types_to_instances.get(ht)
                                                    .expect("Error: type not detected in types_to_instances")
                                                    .clone();

    if chance_to_add == 0 &&
       !instances_name.contains("[") && //TODO: Enable this later
       !instances_name.contains("]") {
      let mut fields : HashMap <String, i32> = HashMap::new();
        for (pf, length) in pf_map.iter() {
          if pf.contains("ttl") || pf.contains("Addr") {
            fields.insert(pf.clone(), 4812389); 
          }
          else {
            fields.insert(pf.clone(), rand::thread_rng().gen_range(0,100));  
        
          }
        }
      packet_data_fields.insert(instances_name, fields);

    }
  }
  let mut metadata_map : HashMap <String, i32> = HashMap::new();
  for mf in metadata_fields.iter() {
    metadata_map.insert(mf.clone(),
                              rand::thread_rng().gen_range(0,100));
  }
  packet_data_fields.insert("standard_metadata".to_string(),
                            metadata_map);
  Packet {
    packet_and_metadata_fields : packet_data_fields,
    active : true,
  }

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


// Simulates dRMT using dgen generated match+action code
fn execute_p4_drmt (args : Vec <String>) 
{
    let drmt_schedule =  match_action_ops::generate_schedule();
    println!("Schedule: {:?}", drmt_schedule);
    let p4_input_file : &str = &args[2];
    let table_entries_file : &str = &args[3];
    let num_packet_fields : i32 = 
    match args[4].parse::<i32>() {

        Ok  (t_pkts)    => t_pkts,
        Err (_)         => panic!("Failure: Unable to unwrap num_packet_fields"),
      };

    let ticks : i32 = 
      match args[5].parse::<i32>() {

        Ok  (t_ticks) => t_ticks,
        Err (_)       => panic!("Failure: Unable to unwrap ticks"),
      };
    let num_processors : i32 = 
      match args[6].parse::<i32>() {

        Ok  (t_num_processors)    => t_num_processors,
        Err (_)                   => panic!("Failure: Unable to unwrap num_processors"),
      };

    let num_state_vars : i32 = 
      match args[7].parse::<i32>() {

        Ok  (t_num_state_vars)   => t_num_state_vars,
        Err (_)                   => panic!("Failure: Unable to unwrap num_state_vars"),
      };
    let mut processors : Vec<dRMTProcessor> = Vec::new();

    assert! (ticks >= 1);
    let drmt_schedule_keys : Vec<i32>= 
      drmt_schedule.iter()
                   .map(|(k, _)| *k)
                   .collect();
    let ticks_to_complete : i32 = 
      drmt_schedule_keys.iter()
                        .fold(0, |x, y| {
        // Get max value in keys
        if x >= *y {
          x
        }
        else {
          *y
        }
    }) + match_action_ops::get_action_ticks();
    let table_entries_map : HashMap <String, Vec<MatchAction>> = 
      drmt_utils::parse_table_entries(table_entries_file);
    let mut stateful_memories : StatefulMemories = 
      drmt_utils::init_stateful_memories(match_action_ops::registers(),
                                         match_action_ops::counters(),
                                         match_action_ops::meters());
    println!("table entries: \n{:?}", table_entries_map);
    println!("{:?}", stateful_memories.memories);
    for p in 0..num_processors {
      processors.push(dRMTProcessor { processor_id : p,
                                      schedule : drmt_schedule.clone(),
                                      table_name_to_actions : match_action_ops::table_name_to_actions(),
                                      packets_and_initial_tick  : Vec::new(),
                                      current_tick : -1,
                                      packet_output_strings : HashMap::new(),
                                      tick_duration : ticks_to_complete,
                                      entries_to_populate : table_entries_map.clone(),
                                      call_action : match_action_ops::get_call_action_function(),
      });

    }
    let header_types : HashMap <String, HashMap<String,i32>> = 
        match_action_ops::header_types();
    let types_to_instances : HashMap <String, String> = 
        match_action_ops::types_to_instances();
    let mut output_string : String = "".to_string();
    for t in 0..ticks {
           processors[(t % num_processors) as usize].add_packet(
           generate_random_packet(header_types.clone(),
                                  types_to_instances.clone()), t);
       for p in processors.iter_mut() {
         output_string.push_str(&p.tick(&mut stateful_memories));
       }
    }
   fs::write("output_drmt".to_string(), output_string); 
   println!("dRMT output written to output_drmt");
}

#[warn(unused_imports)]
fn main() {

  let args : Vec<String> = env::args().collect();
  let arch : &str = &args[1];
  println!("Args: {:?}", args);
  match arch {
    "PISA"  => {
                assert!(args.len() == 4 || args.len() == 5);
                execute_rmt(args);
    },
    "pisa"  => {
                assert!(args.len() == 4 || args.len() == 5);
                execute_rmt(args);
    },
    "drmt_p4" => {
                assert!(args.len() == 8);
                execute_p4_drmt(args);

    },
    _      => panic!("Incorrect architecture. Only rmt and drmt are supported"),
  };

}

#[cfg(test)]
mod test_druzhba;
mod test_with_chipmunk;
