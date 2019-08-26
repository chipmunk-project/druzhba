// Represents the Alu that's read from the
// .alu files

pub struct AluParsingUtils {

  pipeline_stage : i32,
  alu_count : i32,
  name : String,
  alu_body : String,
  is_stateful : bool,
}

impl AluParsingUtils {

  pub fn new (t_pipeline_stage : i32,
              t_alu_count : i32,
              t_name : String,
              t_alu_body : String,
              t_is_stateful : bool,) -> Self {
    AluParsingUtils {
      pipeline_stage : t_pipeline_stage,
      alu_count : t_alu_count,
      name : t_name, 
      alu_body : t_alu_body,
      is_stateful : t_is_stateful,
    }
  }
  
  // Returns the correct formatted string to be parsed
  // by LALRPOP
  pub fn prepend_opt_header_to_alu (&self) -> String{
    let name_header : String = 
        format! ("name : {}\n", self.name);
    let pipeline_stage_header : String = 
        format! ("pipeline stage : {}\n", self.pipeline_stage);   
    let alu_count_header : String = 
        format! ("alu  : {}\n", self.alu_count);
    format! ("{}{}{}{}", name_header, 
                         pipeline_stage_header, 
                         alu_count_header, 
                         self.alu_body) 
  }
  pub fn get_number_of_operands (&mut self) -> i32{
    let alu_body_line : Vec<String> = self.alu_body
                                        .split("\n")
                                        .map (|s| s.to_string())
                                        .collect();

    match self.is_stateful {
      true  => {
        let pkt_vec : Vec <&str> = alu_body_line [3].split(",").collect();
        pkt_vec.len() as i32
      },
      false => {  
        let pkt_vec : Vec <&str> = alu_body_line [4].split(",").collect();
        pkt_vec.len() as i32
      }
    }
  }
  pub fn get_number_of_state_variables (&self) -> i32 {
    match self.is_stateful {
      true => {
        let alu_body_line : Vec<String> = self.alu_body
                                              .split("\n")
                                              .map (|s| s.to_string())
                                              .collect();
        let state_vec : Vec <&str> = 
            alu_body_line [1].split(",").collect();
        state_vec.len() as i32
      },
      false => 0,
    }
  }
  pub fn increment_pipeline_stage (&mut self) {
    self.pipeline_stage += 1;
  }
  pub fn increment_alu_count (&mut self) {
    self.alu_count += 1;
  }
  pub fn reset_alu_count (&mut self) {
    self.alu_count = 0;
  }
  pub fn reset_alu_body (&mut self) {
    self.alu_count = 0;
  }
}
