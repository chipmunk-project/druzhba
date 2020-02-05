use std::fs;
use std::collections::HashMap;
pub struct dRMTCodeGenerator {
    pub input_file : String,
    pub output_file : String,
}

impl  dRMTCodeGenerator{
    fn generate_use_declarations (&self) -> String {
        String::from("use std::collections::HashMap;\n")
    }
    // Generates code to create hashmap of schedule
    fn generate_hashmap_schedule (&mut self, 
                                  schedule : HashMap <i32, Vec<String>>) -> String {
        let mut generate_schedule_string : String = 
           String::from("pub fn generate_schedule() -> HashMap <i32, Vec<String>> {\n  let schedule : HashMap <i32, Vec<String>> = HashMap::new();\n");
        for (key, value) in schedule.into_iter() {
          if value.len() > 0 {
            generate_schedule_string.push_str(
             &format!("  schedule.insert({}, {:?});\n", key, value) );
          }
        }
        generate_schedule_string.push_str("  schedule\n}\n");
        generate_schedule_string
    }

    fn separate_by_whitespace (&self, s : String) -> Vec <String> {
      s.split_whitespace()
       .into_iter()
       .map(|s| s.to_string())
       .collect()
    }
    // Opens P4 file and parses actions, matches, and headers.
    // Converts them into Rust code
    fn parse_p4_file (&mut self) -> String {
      let p4_file_contents : String = fs::read_to_string(&self.input_file)
                                      .expect("Error: could not read from P4 file");
      let p4_file_lines : Vec<String> = p4_file_contents
                                        .split("\n")
                                        .map(|s| s.to_string())
                                        .collect(); 

      let mut curly_brace_stack : Vec<&str> = Vec::new();
      let mut state : &str = "none";
      let mut ignore : bool= false;
      for i in 0..p4_file_lines.len() {

        let line : String = p4_file_lines[i].clone();
        let mut function_string : String = String::from("");
        let mut line_vec : Vec<String> = 
           self.separate_by_whitespace(line);
        if line_vec.contains(&String::from("{")) {
          curly_brace_stack.push("{");
        }
        if line_vec.contains(&String::from("}")) {
          curly_brace_stack.pop();
        }

        // If in multiline comment, just skip
        if state == "comment" {
          if line_vec[line_vec.len() - 1] == "*/" {
            state = "none";
          }
          continue;
        }
        // Do not care about parser and control 
        else if state == "parser" ||
                state == "control" {
          if curly_brace_stack.len() == 0 {
            state = "none";
          } 
          continue;
        }
        if line_vec.len() == 0 || 
           line_vec[0] == "//" {
          continue; 
        }
        // Skip multiline comments
        else if line_vec[0] == "/*" {
          if line_vec [line_vec.len() -1] != "*/" {
            state = "comment"; 
          } 
          continue;
        }
        // Do not need control or parser so skip these lines
        else if line_vec[0] == "control" {
        
          assert!(line_vec[line_vec.len() - 1] == "{");
          state = "control";
          curly_brace_stack.push("{");
          continue;
        }
        else if line_vec.len() > 1 && 
                line_vec[1] == "parser" {

          assert!(line_vec[line_vec.len() - 1] == "{");
          state = "parser";   
          curly_brace_stack.push("{");
          continue;
        }

 /*
        else if line_vec[0] == "table" {
          i+=1;
          line_vec[i] = self.separate_by_whitespace 
                        (p4_file_lines[i].clone());
 
        }*/
        println!("{:?}", line_vec);
       }
       String::from("")
    }


    pub fn generate (&mut self, 
                     schedule : HashMap <i32, Vec<String>>) {
        self.parse_p4_file();
/*
        let file_string : String = format!("{}{}",
                                           self.generate_use_declarations(),
                                           self.generate_hashmap_schedule(schedule) );
        fs::write(self.output_file.clone(), 
                  file_string);*/

    }

}
