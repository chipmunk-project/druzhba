use crate::phv_container::PhvContainer;
use std::collections::HashMap;
use std::ops::{Index, IndexMut, AddAssign};
use std::fmt;


pub type FieldName = String;
pub type FieldType = i32;
pub type PacketFieldSet = Vec <String>;


#[derive(Clone)]
pub struct Phv<T> {
  pub bubble : bool, // false if initialized, true otherwise
  pub packets : Vec<PhvContainer<T>> // Vector of PHV Containers
}

/*different PhvContainer types should be allowed in the PHV*/

impl<T> Phv<T>{

  pub fn new() -> Self {
    Phv{ bubble : true, packets: Vec::new()}
  }

  pub fn new_with_pack(pack : PhvContainer<T>) -> Self {
    let list : Vec<PhvContainer<T>> = vec![pack];
    Phv{ bubble : false, packets : list }
  }

  pub fn is_bubble(&self) -> bool {
    self.bubble == true
  }

  pub fn add_container_to_phv(&mut self, pack: PhvContainer<T>) -> &Self {
    self.packets.push(pack);
    self.bubble = false;
    self
  }
}

/*No need for AddAssign Trait implementation for PHV's - there is only one phv per pipeline stage*/

impl<T> Index<i32> for Phv<T>{
  type Output = PhvContainer<T>;
  fn index(&self, i : i32) -> &Self::Output {
    &self.packets[i as usize]
  }
}

impl<T> IndexMut<i32> for Phv<T> {
  fn index_mut(&mut self, i: i32 ) -> &mut PhvContainer<T> {
    &mut self.packets[i as usize]
  }
}

impl<T> fmt::Display for Phv<T> where T : fmt::Display {

  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let mut s : String = String::from(""); 
      let mut counter = 0;
      for container in &self.packets {
        write!(f, "\nindex : {}, value : {}\n", &counter.to_string(), &self.packets[counter].field_value.to_string());
        counter += 1;
      }
      write!(f, "{}", s)
  }
}




