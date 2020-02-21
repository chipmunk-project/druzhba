use crate::packet::Packet;

struct MatchAction {
  pub match_table : String,
  pub match_header_type : String,
  pub match_field : String,
  pub match_type : String, // lpm, ternary, exact, or range
//  pub match_range_begin : String,
 // pub match_range_end : String, // -1 if exact match
  pub action_name : String,
}

impl MatchAction {

  pub fn is_match (&self, incoming_packet : &mut Packet) -> bool {
    if incoming_packet.contains_field(&self.match_header_type,
                                      &self.match_field){
      let input_field = incoming_packet.get_field_value(&self.match_header_type,
                                                        &self.match_field);
      /*
      if match_range_end == -1 {
         
      }*/

    }
    // TODO: Complete
    true
  }
  pub fn get_action (&self) -> &str {
    &self.action_name
  }
}
