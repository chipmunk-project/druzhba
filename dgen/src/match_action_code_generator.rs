use std::fs;
use std::collections::HashMap;
pub struct MatchActionCodeGenerator {
    pub input_file : String,
    pub output_file : String,
}

impl  MatchActionCodeGenerator{

    fn generate_use_declarations (&self) -> String {
        let use_hashmap : String = 
          String::from("use std::collections::HashMap;\n"); 
        let use_packet : String = 
          String::from("use druzhba::packet::Packet;\n"); 
        let use_stateful_memories : String =
          String::from("use druzhba::stateful_memory::{StatefulMemory, StatefulMemories};\n");
        format!("{}{}{}", use_hashmap,
                          use_packet, 
                          use_stateful_memories)
 
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

    // Takes in the included P4 file and modifies it so that
    // the file can be found from the current directory
    fn add_new_file (&self, 
                     token : String,
                     complete_p4_files : &mut Vec<String>) {
       // Strip the quotes
       let new_file : String = token[1..token.len() - 1].to_string();
       let path_vec : Vec <String> = self.input_file
                                     .split("/")
                                     .map (|s| s.to_string())
                                     .collect();
       if path_vec.len() == 1 {
         complete_p4_files.push(new_file);
       } 
       else {
         let mut altered_p4_file : String = String::from("");
         for j in 0..path_vec.len() - 1 {
           if path_vec[j] == "" && j == 0 {
             altered_p4_file.push_str("/"); 
           }
           else {
             altered_p4_file.push_str
               (&format!("{}/", path_vec[j]));
           }
         }
         complete_p4_files.push(format!("{}{}",
                                         altered_p4_file,
                                         new_file).to_string());
      }
    }

    // Extracts match information from table reads.
    // Gets the header_type being matched, the field, and
    // the type of match
    fn parse_token_table_reads (&self,
                                token : String,
                                next_token : String,
                                prev_token : String,
                                table_stack : &mut Vec<String>,
                                tmp_token_buffer : &mut Vec<String>,
                                table_matches_string : &mut String)
    {

      if token.contains (".") &&
         next_token == ":"{
          // Skip the period and add two separate vector
          // entries for header_type and field
          let period_index : usize = 
            match token.as_str().find('.') {
              Some (x) => x,
              _        => panic!("Error: string doesn't have a period as expected"),
            };

          let header_type_for_match : String = 
            token[0..period_index].to_string();
          let field_for_match : String = 
            token[period_index+1..token.len()]
                              .to_string();
          tmp_token_buffer.push(header_type_for_match);
          tmp_token_buffer.push(field_for_match);

     }
     else if next_token == ":" {
       tmp_token_buffer.push(token);
     }
     else if prev_token == ":" && 
             next_token == ";" {
       tmp_token_buffer.push(token);
     }
     else if token == ";" {
        let current_table : &str = 
           match table_stack.last() {
           Some (t) => t,
           None     => panic!("Error: table stack is empty but state is table"),

         };
         table_matches_string.push_str(
           &format!("  {}_vec.push(vec![", current_table));

         for token in tmp_token_buffer.iter() { 
           table_matches_string.push_str(&format!("String::from(\"{}\"), ", token));
         }
         table_matches_string.push_str("]);\n");
       tmp_token_buffer.clear();
      }

    } 
    // Parse header_type fields
    fn parse_header_type_fields (&self,
                                 token : String,
                                 next_token : String,
                                 prev_token : String,
                                 tmp_token_buffer : &mut Vec<String>,
                                 header_types : &mut Vec<String>,
                                 header_type_string : &mut String) {
           if token != ":" &&
              token != ";" {
            if next_token == ":" {
              tmp_token_buffer.push(token);
            }
            else if next_token == ";" &&
                    prev_token == ":" {
            let header_type_name = match header_types.last() {
              Some (ht) => ht,
              _         => panic!("Error: header_types vector is empty"),
            };

              header_type_string.push_str(&format!("  {}_map.insert(String::from(\"{}\"), {});\n",
                                                   header_type_name,
                                                   tmp_token_buffer[0],
                                                   token));
              tmp_token_buffer.clear();
                                          
            }
        }
    }
     // Splits string using a given delimiter and returns
    // a Vec<String>
    fn split_string (&self, 
                     current_string : String,
                     delimiter : String) -> Vec<String> {
      current_string.split(delimiter.as_str())
                    .map(|s| s.to_string())
                    .collect() 
      
    }
    // Splits a string using the given delimiter but still
    // includes them in the final vector ensuring that they
    // are in the same place that they were in the original
    // string. For every one of these string items, divide them
    // further in the same way if they contain any other 
    // potential delimiters.
    //
    // Example: 
    //   drop(); => ["drop", "(", ")", ";"]
    fn divide_into_vec (&self, 
                        current_string : String,
                        delimiter : String) -> Vec<String> {
  
      let mut ret_vec : Vec<String> = Vec::new();
      let string_vec : Vec<String> = self.split_string(
                                          current_string.clone(),
                                          delimiter.clone());
      for i in 0..string_vec.len() {
        let s : String = string_vec[i].clone();
        let mut s_simplified = self.simplify_string(s);
        ret_vec.append(&mut s_simplified);
        if i < string_vec.len() - 1{
          ret_vec.push(delimiter.clone());
        }
     
      }
      ret_vec
    }
             
    // Initiates string splitting if the string contains any 
    // important tokens we would like to separate        
    fn simplify_string (&self, 
                        current_string : String) -> Vec<String> {

      if current_string.contains ("(") {
        self.divide_into_vec(current_string.clone(),
                             "(".to_string())
      }
      else if current_string.contains (")") {
        self.divide_into_vec(current_string.clone(),
                             ")".to_string())
      }
      else if current_string.contains ("{") {
        self.divide_into_vec(current_string.clone(),
                             "{".to_string())
      }
      else if current_string.contains ("}") {
        self.divide_into_vec(current_string.clone(),
                             "}".to_string())
      }
      else if current_string.contains (",") {
        self.divide_into_vec(current_string.clone(),
                             ",".to_string())
      }
      else if current_string.contains("/*"){
        self.divide_into_vec(current_string.clone(),
                             "/*".to_string())
      }
      else if current_string.contains("*/"){
        self.divide_into_vec(current_string.clone(),
                             "*/".to_string())
      }
      else if current_string.contains("//"){
        self.divide_into_vec(current_string.clone(),
                             "//".to_string())
      }
      else if current_string.contains(";"){
        self.divide_into_vec(current_string.clone(),
                             ";".to_string())
      }

      else if current_string == "" {
        Vec::new()
      }
      else {
        vec![current_string]
      }
    }

    // Takes in a P4 file and performs tokenization.
    // Returns a vector of tokens that are used for Rust code
    // generation
    fn lexer (&self,
              file : String) -> Vec <String> { 
      let p4_file_contents : String = fs::read_to_string(file)
                                      .expect("Error: could not read from P4 file");
      let p4_file_lines : Vec<String> = p4_file_contents
                                        .split("\n")
                                        .map(|s| s.to_string())
                                        .collect(); 
      let mut tokens : Vec<String> = Vec::new();
      // Iterate thorugh lines of P4 file
      for i in 0..p4_file_lines.len() {

        let line : String = p4_file_lines[i].clone();
        let line_vec : Vec<String> = 
           self.separate_by_whitespace(line);
        for elem in line_vec.iter () {
          let mut elem_vec : Vec<String> = self.simplify_string(elem.to_string());
          if elem_vec.len() > 0 {
            tokens.append(&mut elem_vec);    
          }
          else {
            tokens.push(elem.to_string()); 
          }
        }
      }
      tokens
    }
    // Takes in a list of actions, HashMap of action anems to tokens,
    // and types the parameters take. Takes each action in the list
    // and generates the functions. Then receives a list of new
    // non-primitive functions that were called and generates
    // those as well
    fn compound_actions_string (&self,
                                actions_list : Vec<String>, // Root actions from tables
                                actions_to_strings_map : HashMap <String, Vec<String>>, // Action name to tokens
                                argument_types : HashMap<String, Vec<String>>,  // Codes for the arg types (reference or value)
                                is_table_action : bool)
                                -> String {

      
      let mut function_actions_string : String = String::from("");
      for a in actions_list.iter() {
        let tokens : Vec<String> = 
          match actions_to_strings_map.get(a) {
            Some (t) => t.clone(),
            _        => panic!("Error: actions_to_strings_map doesn't contain a table action"), 
          };
        let argument_types_for_action : Vec<String> = 
          argument_types.get(a).expect("Error: argument_types does not contain table action").clone();
        let (function_string, new_argument_types) : 
            (String , HashMap <String, Vec<String>>) = 
                        self.process_function(tokens, 
                                              argument_types_for_action,
                                              is_table_action);
        function_actions_string.push_str(&function_string);

        let mut new_actions_list : Vec<String>= Vec::new();
        // BFS on processing functions that were called within the
        // original function 
        for k in new_argument_types.keys() {
          new_actions_list.push (k.clone());
        }
        function_actions_string.push_str(
                                 &self.compound_actions_string (new_actions_list,
                                                                actions_to_strings_map.clone(),
                                                                new_argument_types.clone(),
                                                                false));
        }                                        
      
      function_actions_string
    }
    // Takes tokens of an action and a vector containing the 
    // types of the parameters to be taken in. Returns String
    // of the transpiled Rust code
    fn process_function (&self,
                        tokens : Vec<String>,
                        argument_types : Vec<String>,
                        is_table_action : bool) 
                         -> (String, HashMap <String, Vec<String>>){
      let mut curly_brace_stack : Vec<String> = Vec::new();
      let mut action_functions_header : String = String::from("");
      let mut action_functions_body : String = String::from("");
      let mut argument_num : usize = 0;
      let mut new_argument_types : HashMap <String, Vec<String>> = HashMap::new();
      let mut parameter_types : HashMap <String, String> = HashMap::new();
      let mut calling_function_name : String = String::from("none");  
      let mut calling_function_types : Vec<String> = Vec::new();
      // "special" primitive functions take in pkt as param
      let mut is_special_primitive : bool = false;
      let mut field_declarations : String = String::from("");
      let mut field_ctr : i32 = 0;
      let mut header_types_and_fields : Vec<(String, String)> = Vec::new();

        // Other actions that are not in the table and are not primitive
        for i in 0..tokens.len() {
          let token : String = tokens[i].clone();
          
          if i == 0 {
            action_functions_header.push_str(&format!("fn {}", token));
            continue;
          }
          if token == "{" {
            curly_brace_stack.push("{".to_string());
          }
          else if token == "}" {
            curly_brace_stack.pop();
          }
          // Append type after parameter name and add Packet
          // parameter
          if curly_brace_stack.len() == 0 &&
             (token == "," || token == ")") &&
              !(token == ")" && tokens[i-1] == "(") {
              let arg_type : &str= &argument_types[argument_num];

              match arg_type {
                "reference" => {
                  action_functions_header.push_str(" : &mut i32 ");
                  argument_num+=1;
                  parameter_types.insert(tokens[i-1].clone(), "reference".to_string());
                },
                "value"     => {
                  action_functions_header.push_str(" : i32 ");
                  argument_num+=1;
                  parameter_types.insert(tokens[i-1].clone(), "value".to_string());
                },

                "root"      => {
                  action_functions_header.push_str(" : i32 ");
                  parameter_types.insert(tokens[i-1].clone(), "value".to_string());
                },
                _           => panic!("Error: Invalid argument type provided "), 
              }
              if token == ")" {
                  action_functions_header.push_str(", pkt : &mut Packet, stateful_memories : &mut StatefulMemories"); 
              }
              action_functions_header.push_str(&token);
          }
          // Add packet parameter for for actions with no parameters
          else if curly_brace_stack.len() == 0 &&
                  (token == ")" && tokens[i-1] == "(") {

            action_functions_header.push_str("pkt : &mut Packet,  stateful_memories : &mut StatefulMemories)");

          }
          else if token=="{" {
            action_functions_header.push_str("{\n  ");
          }
          else if token == ";" {
            if calling_function_name != "none" {
              new_argument_types.insert(calling_function_name.clone(),
                                        calling_function_types.clone());
            }
            calling_function_types.clear();
            calling_function_name = "none".to_string();
            action_functions_body.push_str(";\n  ");
          }
          // TODO: Update this when new primitive functions are supported
          else if curly_brace_stack.len() != 0 &&
                  (tokens[i-1] == ";" ||
                  tokens[i-1] == "{") {
    
            if token != "add_to_field" &&
               token != "modify_field" &&
               token != "drop"         &&
               token != "count" {
              calling_function_name = token.clone();
            }
            else {
             calling_function_name="none".to_string();
            }
            action_functions_body.push_str(&token);
            if token == "drop" {
              is_special_primitive = true;
            }
          }
          // NOTE: currently not processing hex values. Look into
          // implementing this if masks are needed. Used to prevent
          // working with hex values for modify_field action
          else if token.len() > 2 && 
                  token[0..2].to_string() == "0x" {
            continue;
          }
          // Handle packet or metadata field
          else if token.contains(".") {
              let period_index : usize = 
                match token.as_str().find('.') {
                  Some (x) => x,
                  _        => panic!("Error: action string doesn't have a period as expected"),
            };

              let header_type_for_action : String = 
                token[0..period_index].to_string();
              let field_for_action : String = 
                token[period_index+1..token.len()]
                                  .to_string();

           
            field_declarations.push_str(
              &format!("  let mut field_{} : i32 = pkt.get_field_value (\"{}\", \"{}\");\n  ",
                      field_ctr, &header_type_for_action, &field_for_action));
            action_functions_body.push_str(
               &format!("&mut field_{}", field_ctr));
            header_types_and_fields.push((header_type_for_action.clone(),
                                         field_for_action.clone()));
            field_ctr += 1; 
              if calling_function_name != "none" {
                calling_function_types.push("reference".to_string());
              }
            }
            else {


              if !token.parse::<f64>().is_ok() &&
                  curly_brace_stack.len() != 0 &&
                 !parameter_types.contains_key(&token) &&
                 token != "," && 
                 token != "{" && 
                 token != "}" && 
                 token != "(" && 
                 token != ")"  {
                action_functions_body.push_str(
                  &format!("stateful_memories.get_mut_ref_memory(\"{}\")", token));
              }
              // The drop primitive takes in a pkt
              else if is_special_primitive && token == "(" {
                action_functions_body.push_str("(pkt");
              }
              // Add pkt, stateful_memories arguments for calling 
              // compound functions
              else if token == ")" &&
                      calling_function_name != "none" {
                action_functions_body.push_str(", pkt, stateful_memories)");
              }
              else {
                if curly_brace_stack.len() == 0 {
                  action_functions_header.push_str(&token);
                }
                else {
                  action_functions_body.push_str(&token);
                }
              }
              if token != "," && 
                 token != "{" && 
                 token != "}" && 
                 token != "(" && 
                 token != ")" && 
                 calling_function_name != "none" {
                  // Trace parameter type to select whether new function takes in reference or
                  // value
                  if parameter_types.contains_key(&token) {
                    calling_function_types
                              .push(parameter_types
                              .get(&token)
                              .expect("Error: parameter_types map does not contain token")
                              .clone()); 
                  }
                  else {
                    calling_function_types.push("value".to_string());
                  }
              }
            }
          }

      for i in 0..field_ctr {
        let pair : (String, String) = 
          header_types_and_fields[i as usize].clone();
        action_functions_body.push_str(&format!("  *pkt.get_mut_ref_field(\"{}\", \"{}\") = field_{};\n", 
                                       pair.0, pair.1, i));
      }
      action_functions_body.push_str("\n}\n");
      (format!("{}{}{}", action_functions_header,
                         field_declarations,
                         action_functions_body),
      new_argument_types)
    }
    // Code generation for meters, registers, counters.
    // Each stateful memory is represented as a HashMap
    fn update_stateful_strings (&self,
                                state : &str,
                                name_vec : &mut Vec<String>,
                                current_string : &mut String,
                                prev_token : &str,
                                token : &str, 
                                next_token : &str) {
          if current_string.len() == 0 {
            *current_string = format!("fn {}s () -> HashMap <String, HashMap <String, String>> {{\n  let mut {}_map : HashMap <String, HashMap <String,String>> = HashMap::new();\n", state, state);
          }
          if prev_token == state {
            current_string.push_str(&format!("  let mut {}_map : HashMap <String, String> = HashMap::new();\n", &token));
            name_vec.push (token.to_string());
          }
          else if token == ":" {

            let name : String = name_vec.last().expect("Error: name_vec is empty").clone();
            current_string.push_str(&format!("  {}_map.insert(String::from(\"{}\"), String::from(\"{}\"));\n", 
                                     &name, prev_token, next_token));
          }

    }         
    // Parse tokens from P4 file and generates Rust code String.
    // Returns a pair; the first element is the Rust string to
    // be written to a new file and the second is a Vec<String>
    // which contains any new P4 files to parse next that the 
    // current P4 file includes
    fn extract_p4_contents (&self, 
                           tokens : Vec<String>) -> 
                           (String, Vec<String>) {

      // These two structures are for caching action data
      let mut actions_list : Vec<String> = Vec::new();
      let mut actions_to_strings_map : HashMap <String, Vec<String>> = 
          HashMap::new();
      let mut table_actions_list : Vec<String> = Vec::new();
      let mut curly_brace_stack : Vec<String> = Vec::new();
      let mut header_types : Vec<String> = Vec::new();
      let mut table_stack : Vec<String> = Vec::new();
      let mut register_stack : Vec<String> = Vec::new();
      let mut meter_stack : Vec<String> = Vec::new();
      let mut counter_stack : Vec<String> = Vec::new();
      // state variable indicates which portion of the P4
      // file we are parsing so we know what type of
      // data to extract
      let mut state : String = String::from("none");
      let mut register_string : String = String::from("");
      let mut meter_string : String = String::from("");
      let mut counter_string : String = String::from("");
      let mut header_type_string : String = String::from("");
      let mut action_functions_string : String = String::from("");
      let mut table_matches_string : String = String::from("");
      let mut table_actions_string : String = String::from("");
      let mut complete_p4_files : Vec <String> = Vec::new();
      let mut tmp_token_buffer : Vec <String> = Vec::new(); 
      let mut instances_to_types_string : String = String::from("");
      for i in 0..tokens.len() {
        let token : String= tokens[i].clone();
        // Note: single line comments are not yet being parsed
        if token == "/*" {
          state = String::from("comment");
        }       
        // Completes comment 
        else if token == "*/" {
          state = String::from("none");
        }

        // Keeps track of curly braces so we know when to
        // change state
        else if token == "{" {
          curly_brace_stack.push(String::from("{"));
        }
        else if token == "}" {
          curly_brace_stack.pop();
          if curly_brace_stack.len() == 0 {
            if tmp_token_buffer.len() > 0 {
              tmp_token_buffer.clear();
            }
            state = String::from("none");
          }
        }
        if state == "ignore" {
          continue;
        } 
        else if state == "include" {
  
          self.add_new_file (token.clone(), &mut complete_p4_files);
          state = String::from("none");
        }
        // Enter table section of P4 file
        else if state == "table" {
              if table_matches_string.len() == 0 && 
                 table_actions_string.len() == 0 {
                table_matches_string.push_str(
                  "// Returns HashMap mapping table name with type of match \n"); 
                table_matches_string.push_str(
                  "pub fn matches () -> HashMap<String, Vec<Vec<String>>>{\n");
                table_matches_string.push_str(
                  "  let mut matches : HashMap <String, Vec<Vec<String>>> = HashMap::new();\n");
                table_actions_string.push_str(
                  "// Returns HashMap of table names to the actions contained in them\n");
                table_actions_string.push_str(
                  "pub fn table_name_to_actions () -> HashMap<String, Vec<String>>{\n");
                table_actions_string.push_str(
                  "  let mut table_to_actions : HashMap <String, Vec<String>> = HashMap::new();\n");

              }

          if tokens[i-1] == "table" {
            table_stack.push(token.clone()); 
            table_matches_string.push_str(&format!("  let mut {}_vec : Vec<Vec<String>> = Vec::new();\n", token));
            table_actions_string.push_str(&format!("  let mut {}_vec  : Vec <String> = Vec::new();\n", token));
          }
          if token == "reads" {
            state = String::from("table_reads");
          } 
        }   
        // Parse table read section 
        else if state == "table_reads" {
          if token == "actions" {
            state = "table_actions".to_string();
          }
          else {
            self.parse_token_table_reads(token.clone(),
                                         tokens[i+1].clone(),
                                         tokens[i-1].clone(),
                                         &mut table_stack,
                                         &mut tmp_token_buffer,
                                         &mut table_matches_string);

          } 
        }
        // Parse table actions section
        else if state == "table_actions" &&
                token != ";" &&
                token != "{" &&
                token != "}" {
          let current_table : String = match table_stack.last() {
            Some (t) => t.to_string(),  
            _        => panic!("Error: table stack is empty when in table_actions state"),
          };
          // TODO: Consider whether to prevent duplicates
          table_actions_list.push(token.clone());
          table_actions_string.push_str(&format!("  {}_vec.push(String::from(\"{}\"));\n", 
                                                 current_table, 
                                                 token)); 
        }
        // Translates a P4 action directly into a Rust function
        else if state == "action" {
          // New approach. First cache in action functions
          if tokens[i-1] == "action" {
            actions_list.push(token.clone());
            actions_to_strings_map.insert(token.clone(), 
                                          vec![ token.clone()]); 
          }
          else {  
            let action_name : String = 
              match actions_list.last(){
                Some (a) => a.to_string(),
                _        => panic!("Error: actions_list is empty"),
            };
            let mut current_mapped_vec : Vec<String> = 
              match actions_to_strings_map.get(&action_name) {
                Some (v) => v.clone(),
                _        => panic!("Error: entry not found in actions_to_strings_map"),
              };
            current_mapped_vec.push(token.clone());
            actions_to_strings_map.insert(action_name,
                                          current_mapped_vec); 
          }
        }

        // Get instance names of all header_types and metadata types
        else if state == "header" ||
                state == "metadata" {
          if (tokens[i-1] == "header" ||
              tokens[i-1] == "metadata") &&
              tokens[i+1] != ";" {
            if instances_to_types_string.len() == 0{
              instances_to_types_string.push_str(
                "// Returns HashMap mapping created instance name to the declared header_type or metadata name\n");
              instances_to_types_string.push_str(
                "pub fn instances_to_types () -> HashMap <String, String> { \n");
              instances_to_types_string.push_str(
                "  let mut instances_to_types_map : HashMap <String, String> = HashMap::new();\n");
            }
            instances_to_types_string.push_str(
                &format!("  instances_to_types_map.insert(String::from(\"{}\"), String::from(\"{}\"));\n",
                                                    &tokens[i+1],
                                                    &token));
          }
          if token == ";" {
            state = String::from("none");
          }
        }
        // Parse all of the different header_types
        else if state=="header_type" {
          if tokens[i-1] == "header_type" {
            if header_type_string.len() == 0 {
              header_type_string.push_str(
                "// Returns HashMap from  a header_type to a HashMap of header fields\n");
              header_type_string.push_str(
                "pub fn header_types () -> HashMap <String, HashMap<String, i32> > { \n");
              header_type_string.push_str(
                "  let mut header_types : HashMap<String, HashMap <String, i32>> = HashMap::new();\n");
            }
            header_type_string.push_str(
              &format!("\n  let mut {}_map : HashMap <String, i32> = HashMap::new();\n", 
               token));
            header_types.push(token.clone());
          }
          else if token == "fields" {
            state = String::from("header_type_fields");
          }
        }
        else if state == "header_type_fields" {
          self.parse_header_type_fields (token.clone(),
                                    tokens[i+1].clone(),
                                    tokens[i-1].clone(),
                                    &mut tmp_token_buffer,
                                    &mut header_types,
                                    &mut header_type_string);
        }
        else if state == "counter" {
          self.update_stateful_strings ("counter", 
                                   &mut counter_stack, 
                                   &mut counter_string, 
                                   &tokens[i-1],  
                                   &token, 
                                   &tokens[i+1]);
        }
        else if state == "meter" {
          self.update_stateful_strings ("meter", 
                                   &mut meter_stack, 
                                   &mut meter_string, 
                                   &tokens[i-1],  
                                   &token, 
                                   &tokens[i+1]);       
        }
        else if state == "register" {
          if token == ":" && 
             tokens[i+1] == "saturating" && 
             tokens[i+2] == "," && 
             tokens[i+3] == "signed" {
            self.update_stateful_strings ("register", 
                                     &mut register_stack, 
                                     &mut register_string, 
                                     &tokens[i-1],  
                                     &token, 
                                     "saturating, signed");       

          } 
          else {
            self.update_stateful_strings ("register", 
                                     &mut register_stack, 
                                     &mut register_string, 
                                     &tokens[i-1],  
                                     &token, 
                                     &tokens[i+1]);       
          }

        }
        // We can ignore these for now
        else if (token == "parser" ||
                 token == "control" ||
                 token == "#define") &&
                 state != "comment" {
          state = String::from("ignore");
        }
        else if token == "#include" {
          state = String::from("include");
        }
        // Important P4 states that we want 
        else if (token == "table" ||
                 token == "counter" ||
                 token == "meter" ||
                 token == "metadata" || 
                 token == "register" ||
                 token == "action" || 
                 token == "header" || 
                 token == "header_type") &&
                 state != "comment" {
          state = token.clone();
        }
      }
     // Finishes populating the data structures and complete 
     // returns of functions
     if header_types.len() > 0 {
       for h in header_types.iter() {
         header_type_string.push_str(
            &format!("  header_types.insert(String::from(\"{}\"), {}_map);\n",h,  h));
       }
       header_type_string.push_str("  header_types\n}\n");
     }
     if instances_to_types_string.len() > 0 {
       instances_to_types_string.push_str("  instances_to_types_map\n}\n");
     }
     if table_stack.len() > 0 {
       for t in table_stack.iter() { 
         table_matches_string.push_str(
            &format!("  matches.insert(String::from(\"{}\"), {}_vec);\n",t, t));

         table_actions_string.push_str(
            &format!("  table_to_actions.insert(String::from(\"{}\"), {}_vec);\n", t, t));

       }

       table_matches_string.push_str("  matches\n}\n");
       table_actions_string.push_str("  table_to_actions\n}\n");
     }
     if register_stack.len() > 0 {
       for nn in register_stack.iter() {
         register_string.push_str(
            &format!("  register_map.insert(String::from(\"{}\"), {}_map);\n", nn, nn));
       }
       register_string.push_str("  register_map\n}\n");
     }
     if counter_stack.len() > 0 {
       for cn in counter_stack.iter() {
         counter_string.push_str(
            &format!("  counter_map.insert(String::from(\"{}\"), {}_map);\n ", cn, cn));
       }
       counter_string.push_str("  counter_map\n}\n");
     }
     if meter_stack.len() > 0 {
       for mn in meter_stack.iter() {
         meter_string.push_str(
            &format!("  meter_map.insert(String::from(\"{}\"), {}_map);\n ", mn, mn));
       }
       meter_string.push_str("  meter_map\n}\n");

     }

     let mut table_actions_types_map : HashMap <String, Vec<String>> = 
        HashMap::new();
     for ta in table_actions_list.iter() {
       table_actions_types_map.insert(ta.clone(), vec!["root".to_string()]);
     }
     action_functions_string = 
                self.compound_actions_string(table_actions_list, 
                                             actions_to_strings_map , 
                                             table_actions_types_map,
                                             true);

     (format!("{}{}{}{}{}{}{}{}", action_functions_string,
                                  header_type_string,
                                  instances_to_types_string,
                                  table_matches_string,
                                  table_actions_string,
                                  counter_string,
                                  register_string,
                                  meter_string
                            )
    ,complete_p4_files)


    }
    // Opens P4 file and parses actions, matches, and headers.
    // Converts them into Rust code
    fn parse_p4_file (&mut self,
                      tokens : Vec<String>) -> String {

       let mut complete_p4_string : String = String::from("");
       let (file_string, additional_p4_files) = self.extract_p4_contents (tokens);
       complete_p4_string.push_str(&file_string);
       for s in additional_p4_files.iter() { 
         let new_tokens : Vec<String> = self.lexer(s.clone());
         // NOTE: May need to process these new p4_files in future
         let (new_file_string, _p4_files) = self.extract_p4_contents(new_tokens);

         complete_p4_string.push_str(&new_file_string);
        }
        complete_p4_string
    }

    fn generate_drop (&self) -> String {
      "fn drop(p : &mut Packet) {\n  p.drop();\n}\n".to_string()
    }
    fn generate_modify_field (&self) -> String {
      "fn modify_field (pkt_field : &mut i32, value : i32) {\n  *pkt_field = value;\n}\n".to_string()
    }
    fn generate_add_to_field (&self) -> String {
      "fn add_to_field (pkt_field : &mut i32, value : i32) {\n  *pkt_field += value;\n}\n".to_string()    
    }
    fn generate_count (&self) -> String {
      "fn count (c : &mut StatefulMemory, value : i32) {\n  c[value] += value; \n}\n".to_string()
    }
    fn generate_primitive_actions (&self) -> String {
      format!("{}{}{}{}",
              self.generate_drop(),
              self.generate_modify_field(),
              self.generate_add_to_field(),
              self.generate_count())
    }

    pub fn generate (&mut self, 
                     schedule : HashMap <i32, Vec<String>>) {
      let tokens : Vec<String> = self.lexer(self.input_file.clone());
      let p4_string_data : String = self.parse_p4_file(tokens);


      let file_string : String = format!("{}{}",
                                         self.generate_use_declarations(),
                                         self.generate_hashmap_schedule(schedule));
      fs::write(self.output_file.to_string(), 
                format!("{}{}{}",file_string, 
                                self.generate_primitive_actions(),
                                p4_string_data)).expect("Error: could not write to output Rust file");

    }

}
