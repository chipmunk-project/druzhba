// TODO: File representing a PHV container

extern crate num;
use std::ops::{Index, IndexMut, AddAssign};
use std::fmt;
use std::collections::HashMap;

#[derive(Clone)]
pub struct PhvContainer <T> {

  pub map: HashMap<String,T>,
}

impl<T> PhvContainer<T> {
 
  pub fn new () -> Self{
    let hash : HashMap <String, T> = HashMap::new();
    PhvContainer { map : hash }
  }
  // Alternate constructor that takes in a HashMap for use
  // in initializing PhvContainer.
  pub fn with_map (h : HashMap <String, T>) -> Self{
    PhvContainer { map : h }
  }

  // Returns vector containing all of the field names
  // in the phv_container
  pub fn field_list (&self) -> Vec<String> {

    let mut field_names : Vec <String> = Vec::new();
    self.map.iter().for_each( |(name, _)| 
      field_names.push (name.clone()));

    field_names
  }
}
// Overloads the index operator: [ ]. Enables packet 
// fields to be attained by using pc [fn], where fn
// is the field name and pc is the PhvContainer.
impl <T> Index <&str> for PhvContainer <T>  {

  type Output = T;
  fn index (&self, idx : &str) -> &T{

    assert!(self.map.contains_key(idx));
    &self.map[idx]
  }
}
// Overloads the index operator: [ ]. Enables packet
// field values to be mutated by returning a mutable
// reference. pc [fn] = value where fn is the fieldname,
// value is the new value, and is the PhvContainer.
//
// Default trait used in case user accesses element that
// is not present in map. In this case, it will initialize
// that value with the default value for that type.
impl <T> IndexMut<&str> for PhvContainer <T> 
  where T : Default {
  fn index_mut (&mut self, idx : &str) -> &mut T {
 
    // Matches against an option
    match self.map.get_mut (idx) {

      Some (_) => self.map.get_mut(idx).unwrap(),
      None     =>  {
        // Will get an error if return None, so insert
        // default value as a "placeholder" before 
        // returning mutable reference
        self.map.insert (idx.to_string(), T::default());
        self.map.get_mut(idx).unwrap()
      },
    }
  }
}

// Overloads the += operator. It adds the fields from the
// given PhvContainer into the current PhvContainer assuming
// that there are no conflicting values.
//
// PartialEq trait needed for != operator. Clone needed for
// copying names/values into new PhvContainer. Default needed
// for IndexMut
impl <T> AddAssign for PhvContainer <T>
  where T : Default + Clone + PartialEq {
  fn add_assign (&mut self, t_container : Self){

     let mut field_names : HashMap <String,T> = self.map.clone();
     // Check that the current container's key/value pairs differ
     // from the key/value pairs in t_container. If the same key
     // is in both containers, verify that the values corresponding
     // to that key are the same. If not, then exit.
     t_container.map.iter().for_each (|(name, _)| {
       if self.map.contains_key(name){
         if self[name] != t_container[name] {

           panic!("Values of containers for key {} do not match", name)
         }
       }
     });
     self.map.iter().for_each (|(name, _)| {
       if t_container.map.contains_key(name){
         if self[name] != t_container[name] {

           panic!("Values of containers for key {} do not match", name)
         }
       }
     });

    // Add all values to field_names which will replace current
    // HashMap
    t_container.map.iter().for_each ( |(name, value)|{
      field_names.insert (name.clone(), value.clone());
    });
    *self = PhvContainer { map : field_names };
  }
}

// Uses the fmt functon as part of the Display trait. Allows
// PhvContainer to be printed with println!("{}", p) where p is a 
// PhvContainer. Using Display trait also enables the to_string 
// method to be implemented automatically. Prints the HashMap
// keys along with its corresponding values
//
// fmt::Display needed for allowing values to be converted
// to strings. Will not be able to print if T does not implement
// this trait
impl <T> fmt::Display for PhvContainer <T> 
  where T : fmt::Display {
  fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
      
    let mut s : String = String::from("");
    s.push_str("(");

    for (name, ref value) in self.map.iter() {
      s.push_str(name);
      s.push_str(" : ");
      s.push_str(&value.to_string());
      s.push_str(", ");
    }

    s.push_str(")");
    write!(f, "{}", s)
  }
}

