use crate::packet::Packet;
use std::collections::HashMap;

use std::fmt;

#[derive(Clone)]
pub struct MatchAction {
  pub match_header_type : String,
  pub match_field : String,
  pub match_type : String, // lpm, ternary, exact, or range
  pub match_value : i32,
  pub range : i32, // For range, match on inputs within [match_type, match_type + range]. 
  // Should be 0 otherwise
  pub mask : i32, // Used for defining network prefix of ipv4 or range/ternary
  pub action_name : String,
  pub action_args : Vec<i32>
}

impl MatchAction {

  pub fn is_match (&self, incoming_packet : &Packet) -> bool {
    let mut match_found : bool = false;
    if incoming_packet.contains_field(&self.match_header_type,
                                      &self.match_field){
      let input_field = incoming_packet.get_field_value(&self.match_header_type,
                                                        &self.match_field);
      match_found  = match self.match_type.as_str() {
        "exact" => self.match_value == input_field,
        "range" => input_field >= self.match_value &&
                   input_field <= self.match_value + self.range,
        // TODO: Is the correct implementation for ternary?
        "ternary" => self.match_value & self.mask ==
                     input_field & self.mask,
        "lpm"   => self.match_value & self.mask ==
                     input_field & self.mask,

        _ => false
      };
    }
    match_found
  }
  pub fn get_action (&self) -> &str {
    &self.action_name
  }
  pub fn get_match_type (&self) -> &str {
    &self.match_type
  }
}
impl fmt::Debug for MatchAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let mut s : String = "".to_string();
      s.push_str(&format!(
        "match_header_type : {}, ", self.match_header_type));  
      s.push_str(&format!(
        "match_field : {}, ", self.match_field));  
      s.push_str(&format!(
        "match_type : {}, ", self.match_type));  
      s.push_str(&format!(
        "range : [{}, {}], ", self.match_value, 
        self.match_value + self.range));  
      s.push_str(&format!("mask  {}, ", self.mask));

      s.push_str(&format!("action_name : {}, ", self.action_name));

      s.push_str(&format!("action args : {:?}\n", self.action_args));
      write!(f, "{}", s)
    }
}


