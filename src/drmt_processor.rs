use crate::phv::Phv;
use crate::packet::Packet;
use std::collections::HashMap;

#[derive(Clone)]
pub struct dRMTProcessor {
    pub processor_id : i32,
    pub schedule : HashMap <i32, Vec<String>>,
    pub table_name_to_actions : HashMap <String, Vec<String>>,
    pub packets_and_initial_tick : Vec<(Packet, i32)>,
    pub packet_output_strings : HashMap<i32, String>,
    pub current_tick : i32, // Should be initialized to -1
    pub tick_duration : i32,
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
}
