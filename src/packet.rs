use std::collections::HashMap;
pub struct Packet {
  pub header_types : Vec<String>,
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
  pub fn get_field_value (&self, header_type_str : &str, field_name_str : &str) -> i32 {
    *self.packet_and_metadata_fields.get(header_type_str).expect("Error: current header_type does not exist in packet").get(field_name_str).expect("Error: packet field does not exist for this header_type")

  }
  pub fn get_mut_ref_field (&mut self, header_type_str : &str, field_name_str : &str) -> &mut i32 {
    self.packet_and_metadata_fields.get_mut(header_type_str).expect("Error: current header_type does not exist in packet").get_mut(field_name_str).expect("Error: packet field does not exist for this header_type")
  }
}
