use crate::phv::Phv;
use std::collections::HashMap;

pub struct dRMTProcessor {
    pub processor_id : i32,
    pub schedule : HashMap <i32, Vec<String>>,
    pub phvs_and_initial_tick : Vec<(Phv<i32>, i32)>,
    pub phv_output_strings : HashMap<i32, String>,
    pub state : Vec<i32>,
    pub current_tick : i32, // Should be initialized to -1
    pub max_tick : i32,
}

impl dRMTProcessor {

    pub fn add_phv (&mut self, 
                    input_phv : Phv<i32>,
                    cycle : i32)
    {

      self.phv_output_strings.insert(cycle, format!("Processor {} output:\n{} entered on tick {}\n", 
                    self.processor_id, 
                    input_phv, 
                    cycle));
      let pair : (Phv <i32>, i32) = (input_phv, cycle);
      self.phvs_and_initial_tick.push(pair);
    }
    pub fn tick (&mut self) {
      self.current_tick += 1;

      let mut initial_tick_of_exit_phv : i32 = -1;
      for (phv, initial_tick) in self.phvs_and_initial_tick.iter(){
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
                                              self.phv_output_strings.get(&initial_tick).unwrap(), 
                                              self.current_tick,
                                              tasks);
            self.phv_output_strings.insert(*initial_tick, new_string);
          } 
      }
      if self.current_tick - initial_tick >= self.max_tick {
        initial_tick_of_exit_phv = *initial_tick;
      }

    }
      if initial_tick_of_exit_phv != -1 {
        
        let (final_phv, tick) = self.phvs_and_initial_tick.remove(0);

        println!("Packet completed at tick: {}", self.current_tick);
        let output : String = match self.phv_output_strings.get(&tick){
          Some (s) => s.to_string(),
          _        => panic!("Error: string does not exist in phv_output_strings for given tick"),

        };
        println!("{}", output);
      }
    
  }
}
