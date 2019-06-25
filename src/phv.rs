// TODO: File representing a PHV

use crate::phv_container::PhvContainer;
use std::collections::HashMap;
use std::ops::{Index, IndexMut, AddAssign};
use std::fmt;


pub type FieldName = String;
pub type FieldType = i32;
pub type PacketFieldSet = Vec <String>;


#[derive(Clone)]
pub struct Phv {
  bubble : bool, // false if initialized, true otherwise
  packet : PhvContainer <i32> // Contains packet fields/values
}

impl Phv{
  // Constructor that initializes new Phv with a new PhvContainer.
  // Bool bubble is set to true since values have not been
  // initialized
  pub fn new () -> Self {
    let m : HashMap <FieldName, FieldType> = HashMap::new();
    let c : PhvContainer <i32> = PhvContainer{ map : m };

    Phv { bubble : true, packet : c }
  } 

  // Alternate constructor that takes in a PhvContainer
  pub fn with_container (c : PhvContainer <i32>) -> Self{
    Phv { bubble : false, packet : c}
  }
  pub fn is_bubble (&self) -> bool {
    self.bubble
  }
}
// Overloads the index operator: [ ]. Enables packet 
// fields to be attained by using phv [fn], where fn
// is the field name.
impl Index<&str> for Phv{

  type Output=FieldType;
  fn index (&self, idx : &str) -> &FieldType{
    &self.packet[idx]
  }
}
// Overloads the index operator: [ ]. Enables packet
// field values to be mutated by returning a mutable
// reference. phv [fn] = value where fn is the fieldname
// and value is the new value
impl IndexMut<&str> for Phv {

  fn index_mut (&mut self, idx : &str) -> &mut FieldType {
    &mut self.packet[idx]
  }
}
// Overloads the += operator. If the given packet has been
// initialized (i.e. bubble is false) then it relies upon
// PhvContainer's += operator.
//
// Note that add_assign takes ownership of the given packet
// so ensure to clone it when using += if the ownership
// is intended to be retained.
impl AddAssign for Phv {
  fn add_assign (&mut self, t_packet : Self){

    assert! (!t_packet.is_bubble());
    if self.bubble {
      self.bubble = false;   
    }
    self.packet += t_packet.packet.clone(); 
  }
}

// Uses the fmt functon as part of the Display trait. Allows
// Phv to be printed with println!("{}", p) where p is a 
// Phv. Using Display trait also enables the to_string 
// method to be implemented automatically
impl fmt::Display for Phv {
  fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut s : String = String::from("");
    if self.bubble {
      s.push_str("Bubble: true\n") ;
    }

    s.push_str (&self.packet.to_string());
    write!(f, "{}", s)
  }
}

