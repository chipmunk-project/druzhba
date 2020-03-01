
use std::fmt;
use std::ops::{Index, IndexMut};
use std::collections::HashMap;
#[derive(Clone)]
pub struct StatefulMemory {

  pub memory : String, // register, meter, counter
  pub memory_type : String, // Only for meter, counter (bytes, packet)
  pub instance_count : i32,
  pub direct_or_static : String,
  pub min_width : i32, 
  pub result : i32, // Only for meter. TODO: May have to change the type
  pub width : i32, // Only for register. TODO: consider removing
  pub attributes : String, // Only for register, counter
  pub memory_container : Vec<i32>,
  
}

impl StatefulMemory {
  
}

impl Index<i32> for StatefulMemory{
  type Output = i32;
  fn index(&self, i : i32) -> &Self::Output {
     &self.memory_container[i as usize]
  }
}

impl IndexMut<i32> for StatefulMemory {
  fn index_mut(&mut self, i: i32 ) -> &mut i32 {
    &mut self.memory_container[i as usize]
  }
}


pub struct StatefulMemories {
  pub memories : HashMap <String, StatefulMemory>
}

impl StatefulMemories {
  pub fn get_mut_ref_memory (&mut self, 
                             mem_name : &str) 
                             -> &mut StatefulMemory {
    self.memories.get_mut(mem_name)
      .expect("Error: Could not find memory in StatefulMemories")
  }
  pub fn get_memory (&self,
                     mem_name : &str)
                     -> StatefulMemory {
    self.memories.get(mem_name)
      .expect("Error: Could not find memory in StatefulMemories")
      .clone()
  }
}


