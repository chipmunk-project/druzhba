use crate::phv::Phv;
use crate::packet::Packet;
use crate::match_action::MatchAction;
use std::collections::HashMap;

#[derive(Clone)]
pub struct dRMTProcessor {
    pub processor_id : i32,
    pub schedule : HashMap <i32, Vec<String>>,
    pub table_name_to_actions : HashMap <String, Vec<String>>,
    pub packets_and_initial_tick : Vec<(Packet, i32)>,
    // Incoming tick to string of packet
    pub packet_output_strings : HashMap<i32, String>,
    pub current_tick : i32, // Should be initialized to -1
    pub tick_duration : i32,
    // map from table name to match actions
    pub entries_to_populate : HashMap <String, Vec<MatchAction>>
}

impl dRMTProcessor {

    pub fn add_packet (&mut self, 
                    input_packet : Packet,
                    cycle : i32)
    {

      self.packet_output_strings.insert(cycle, format!("Processor {} output:\n{} entered on tick {}\n", 
                    self.processor_id, 
                    input_packet, 
                    cycle));
                    
      let pair : (Packet, i32) = (input_packet, cycle);
      self.packets_and_initial_tick.push(pair);
    }
    pub fn tick (&mut self) {
      self.current_tick += 1;

      let mut initial_tick_of_exit_packet : i32 = -1;
      for (packet, initial_tick) in self.packets_and_initial_tick.iter(){
        // Perform current_tick - initial_tick to align with the 
        // schedule that is allocated for each packet.
        // The schedule provides a relative schedule for each 
        // packet and does not account for the total tick
        // of the simulator
        if self.schedule.contains_key (&(self.current_tick - initial_tick)){
          let tasks : &Vec <String> = self.schedule
                                          .get(&(self.current_tick - initial_tick))
                                          .unwrap();

          if tasks.len() > 0 {
            // TODO: Execute this list of actions
            let action_names : Vec<String> = 
              self.perform_match_action (&tasks, &packet);
            let new_string : String = format!("{} tick: {} {:?}\n", 
                                              self.packet_output_strings.get(&initial_tick).unwrap(), 
                                              self.current_tick,
                                              tasks);
            self.packet_output_strings.insert(*initial_tick, 
                                              new_string);
          } 
      }
      if self.current_tick - initial_tick >= self.tick_duration {
        initial_tick_of_exit_packet = *initial_tick;
      }

    }
      if initial_tick_of_exit_packet != -1 {
        
        let (final_packet, tick) = self.packets_and_initial_tick.remove(0);

        println!("Packet on processor {}, entered on {}, completed at tick {}", self.processor_id, tick, self.current_tick);
        let output : String = match self.packet_output_strings.get(&tick){
          Some (s) => s.to_string(),
          _        => panic!("Error: string does not exist in packet_output_strings for given tick"),

        };
        println!("{}", output);
      }
    
  }
  fn perform_match_action (&self, 
                           tasks : &Vec<String>, 
                           packet : &Packet ) -> Vec<String> {
    let mut action_names : Vec<String> = Vec::new();
    for task in tasks.iter() {
      if task.contains("MATCH") {
        // Matches are only executed at the same time as actions
        continue;
      }
      let table_name : String = task[0..task.len() - 7].to_string();
      // If scheduler runs into populated table match
      if self.entries_to_populate
             .contains_key(&table_name) {
        let match_actions : Vec<MatchAction> = self.entries_to_populate
                                                   .get(&table_name)
                                                   .unwrap()
                                                   .clone();
        let mut current_ternary_action : (String, i32) = 
          ("".to_string(), 0);
        let mut current_lpm_action : (String, i32) = 
          ("".to_string(), 0);
        for ma in match_actions.iter(){
          if ma.is_match(packet) {
/*
            if ma.action_name == "ternary"  &&
               ma.get_prefix() > current_ternary_action.1 {
              current_ternary_action = (ma.get_action().to_string(),
                                        ma.get_prefix()); 
            }*/
            if ma.action_name == "lpm" &&
              ma.mask > current_lpm_action.1 {
              current_lpm_action = (ma.get_action().to_string(),
                                    ma.mask);
            }
            else {
              action_names.push(ma.get_action()
                              .to_string());
              continue; 
              // TODO: Consider multiple matches on same field
            } 
          }
        }
        if current_lpm_action.1 > 0 {
          action_names.push(current_lpm_action.0);
        }
      }
    }
    action_names 
  }
}
