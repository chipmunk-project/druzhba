use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
pub struct Packet {
  pub packet_and_metadata_fields : HashMap <String, HashMap<String, i32>>,
  // Metadata fields for standard_metadata which is always contained:
  //   ingress_port
  //   packet_length
  //   egress_spec
  //   egress_port
  //   egress_instance
  //   instance_type
  //   parser_status
  //   parser_error_location
  // Read about it in page 25 of https://p4.org/p4-spec/p4-14/v1.0.5/tex/p4.pdf
   pub active : bool,
}

impl Packet {

  pub fn drop (&mut self) {
    self.active = false;
  }  
  pub fn contains_header_type (&self,
                               header_type : &str) -> bool {
    self.packet_and_metadata_fields.contains_key(header_type)
  }
  pub fn contains_field (&self,
                         header_type : &str,
                         field : &str) -> bool {
    if !self.contains_header_type(header_type) {
      false
    }
    else {
      self.packet_and_metadata_fields.get(header_type)
                                     .expect("Error: current header_type does not exist in packet")
                                     .contains_key(field)
    }
  }
  pub fn get_field_value (&self, 
                          header_type_str : &str, 
                          field_name_str : &str) 
                          -> i32 {
    *self.packet_and_metadata_fields
         .get(header_type_str)
         .expect("Error: current header_type does not exist in packet")
         .get(field_name_str)
         .expect("Error: packet field does not exist for this header_type")

  }
  pub fn get_mut_ref_field (&mut self, 
                            header_type_str : &str, 
                            field_name_str : &str) 
                            -> &mut i32 {
    self.packet_and_metadata_fields
        .get_mut(header_type_str)
        .expect("Error: current header_type does not exist in packet")
        .get_mut(field_name_str)
        .expect("Error: packet field does not exist for this header_type")
  }
}

//Allows printing of container values
impl fmt::Display for Packet {

  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut s : String = String::from("");
    for (data_type, fields) in self.packet_and_metadata_fields.iter() {
      s.push_str(&format!("{}\n", data_type));
      for (field_name, value) in fields.iter() {
        s.push_str(&format!("\t{} : {}\n", field_name, value));  
      }
    }
    write!(f, "{}", s)
  }
}
