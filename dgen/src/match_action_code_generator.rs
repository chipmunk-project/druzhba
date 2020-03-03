use std::fs;
use std::collections::HashMap;
use match_action_generation_utils;

pub struct MatchActionCodeGenerator {
    pub input_file : String,
    pub output_file : String,
    pub compound_actions_list : Vec<String>,
}

impl  MatchActionCodeGenerator{


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
    // Does not take length, max_length into account. Can be 
    // done in future work
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
                                                   match token.trim() ==  "*" {
    true => "64".to_string(), // Set to 64 bits by default if using * as header len
    _    => token,

    }));
              tmp_token_buffer.clear();
                                          
            }
        }
    }

    fn init_primitive_param_types_map (&self) -> HashMap<String, Vec<String>> {
      let mut prim_types_map : HashMap <String, Vec<String>> = HashMap::new();
       prim_types_map.insert("modify_field".to_string(),
                             vec!["reference".to_string(),
                                  "value".to_string()]);
       prim_types_map.insert("add_to_field".to_string(),
                             vec!["reference".to_string(),
                                  "value".to_string()]);
       prim_types_map.insert("no_op".to_string(),
                             vec![]);
       prim_types_map.insert("count".to_string(),
                             vec!["reference".to_string(),
                                  "value".to_string()]);
       prim_types_map
    }
    fn is_primitive_action (&self,
                            token : &str) -> bool {
      let string_token : String = token.to_string();
      string_token == "modify_field" ||
      string_token == "drop" ||
      string_token == "add_to_field" ||
      string_token == "count" ||
      string_token == "no_op"

    }
    // Takes in a list of actions, HashMap of action anems to tokens,
    // and types the parameters take. Takes each action in the list
    // and generates the functions. Then receives a list of new
    // non-primitive functions that were called and generates
    // those as well
    fn compound_actions_string (&mut self,
                                actions_list : Vec<String>, // Root actions from tables
                                actions_to_strings_map : HashMap <String, Vec<String>>, // Action name to tokens
                                argument_types : HashMap<String, Vec<String>>,  // Codes for the arg types (reference or value)
                                constants_list : &Vec<String>
                                ) -> (String, HashMap <String, i32>) {
      let mut table_actions_to_num_args : HashMap <String, i32> = HashMap::new();
      let mut function_actions_string : String = String::from("");
      for a in actions_list.iter() {
        if self.compound_actions_list.contains(a) {
          continue;
        }
        self.compound_actions_list.push(a.to_string());
        let tokens : Vec<String> = 
          match actions_to_strings_map.get(a) {
            Some (t) => t.clone(),
            _        => panic!("Error: actions_to_strings_map doesn't contain a table action"), 
          };
        let argument_types_for_action : Vec<String> = 
          argument_types.get(a).expect("Error: argument_types does not contain table action").clone();
        let (function_string, new_argument_types, num_args) : 
            (String , HashMap <String, Vec<String>>, i32) = 
                        self.process_function(tokens, 
                                              argument_types_for_action,
                                              constants_list);
        table_actions_to_num_args.insert(a.clone(), num_args);

        function_actions_string.push_str(&function_string);

        let mut new_actions_list : Vec<String>= Vec::new();
        // Processes functions that were called within the
        // original function 
        for k in new_argument_types.keys() {
          new_actions_list.push (k.clone());
        }

        let string_and_arg_num_map : (String, HashMap <String, i32>) = 
                                                                self.compound_actions_string (new_actions_list,
                                                                actions_to_strings_map.clone(),
                                                                new_argument_types.clone(),
                                                                constants_list);

        function_actions_string.push_str(&string_and_arg_num_map.0);
        table_actions_to_num_args.extend(string_and_arg_num_map.1);
        }                                        
      
      (function_actions_string, table_actions_to_num_args)
    }
    // Takes tokens of an action and a vector containing the 
    // types of the parameters to be taken in. Returns String
    // of the transpiled Rust code. Returns HashMap of name of 
    // new function that is being called to the type parameters 
    // and the and number of parameters
    fn process_function (&self,
                        tokens : Vec<String>,
                        argument_types : Vec<String>,
                        constants_list : &Vec <String>) 
                         -> (String, HashMap <String, Vec<String>>, i32){
      let mut curly_brace_stack : Vec<String> = Vec::new();
      let mut action_functions_header : String = String::from("");
      let mut action_functions_body : String = String::from("");
      // Number of parameters for non-table action, index into argument_types
      let mut argument_num : usize = 0;
      // Number of parameters for table action
      let mut table_action_args_num : usize = 0;
      let mut new_argument_types : HashMap <String, Vec<String>> = HashMap::new();
      let mut parameter_types : HashMap <String, String> = HashMap::new();
      let mut calling_function_name : String = String::from("none");  
      let mut calling_function_types : Vec<String> = Vec::new();
      // "special" primitive functions take in pkt as param
      let mut primitive : String = "".to_string();
      let mut field_declarations : String = String::from("");
      let prim_param_types : HashMap <String, Vec<String>> = self.init_primitive_param_types_map();
      let mut prim_index : usize = 0;
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

              // P4 actions don't explicitly tell what types the
              // parameters are. Need to determines types for 
              // Rust generation. 
              match arg_type {
                "reference_field" => {
                  action_functions_header.push_str(" : &mut i32 ");
                  argument_num+=1;
                  parameter_types.insert(tokens[i-1].clone(), 
                                         "reference_field".to_string());
                },
                "reference_memory" => {
                  action_functions_header.push_str(" : &mut StatefulMemory ");
                  argument_num+=1;
                  parameter_types.insert(tokens[i-1].clone(), 
                                         "reference_memory".to_string());
                },
                "value"     => {
                  action_functions_header.push_str(" : i32 ");
                  argument_num+=1;
                  parameter_types.insert(tokens[i-1].clone(), 
                                          "value".to_string());
                },

                "root"      => {
                  table_action_args_num += 1;
                  action_functions_header.push_str(" : i32 ");
                  parameter_types.insert(tokens[i-1].clone(), 
                                         "value".to_string());
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
            // If action that is being called is a primitive
            // action, no need to process it since it's
            // already being generated 
            if !self.is_primitive_action (&token) {
              calling_function_name = token.clone();
              primitive = "".to_string();
            }
            else {
              calling_function_name = "none".to_string();
              primitive = token.to_string();
              prim_index = 0;
            }
            action_functions_body.push_str(&token);
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
            if primitive.len() > 0 {
              let type_vec : Vec<String> = prim_param_types.get(&primitive)
                                                          .unwrap()
                                                          .to_vec();
              if type_vec[prim_index] == "value" {
                action_functions_body.push_str(
                  &format!("pkt.get_field_value (\"{}\", \"{}\")",
                      &header_type_for_action, &field_for_action));
                prim_index+=1;

              }
              else {
                // Create mutable reference to that field 
                field_declarations.push_str(
                  &format!("  let mut field_{} : i32 = pkt.get_field_value (\"{}\", \"{}\");\n  ",
                          field_ctr, &header_type_for_action, &field_for_action));
                action_functions_body.push_str(
                   &format!("&mut field_{}", field_ctr));
                header_types_and_fields.push((header_type_for_action.clone(),
                                             field_for_action.clone()));
                prim_index+=1;
                field_ctr += 1; 
                }
              }
              else {
                calling_function_types.push("reference_field".to_string());
              }
            }
            else {
              // Checks if this token is a number, a 
              // parameter, or a constant. Also checks if we
              // are not in the functon header. If not, then
              // this token must be a stateful memory
              if !token.parse::<f64>().is_ok() &&
                  curly_brace_stack.len() != 0 &&
                 !parameter_types.contains_key(&token) &&
                 !constants_list.contains(&token) &&
                 token != "," && 
                 token != "{" && 
                 token != "}" && 
                 token != "(" && 
                 token != ")"  {
                field_declarations.push_str(
                  &format!("  let mut field_{} : StatefulMemory =  stateful_memories.get_memory(\"{}\");\n  ", 
                    field_ctr, token));
                action_functions_body.push_str(
                  &format!("&mut field_{}", field_ctr));
                header_types_and_fields.push(("".to_string(), token.clone())); 
                field_ctr+=1;
                prim_index+=1;
                if primitive.len() == 0 &&
                   calling_function_name != "none"{
                  calling_function_types.push("reference_memory".to_string());
                }
              }
              // The drop primitive takes in a pkt
              else if primitive == "drop" && token == "(" {
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
                  if token.parse::<f64>().is_ok() &&
                     primitive.len() > 0 {
                    prim_index+=1; 
                  } 
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
        if pair.0 != "" {
          action_functions_body.push_str(&format!("  *pkt.get_mut_ref_field(\"{}\", \"{}\") = field_{};\n", 
                                       pair.0, pair.1, i));
        }
        else {
          action_functions_body.push_str(&format!("  *stateful_memories.get_mut_ref_memory(\"{}\") = field_{};\n", 
                                    pair.1, i));

        }
      }
      let mut args_count : i32 = 0;
      if table_action_args_num > 0 {
        args_count = table_action_args_num as i32;
      }
      else if argument_num > 0 {
        args_count = argument_num as i32; 
      }
      else {
        args_count = 0;
      }
      action_functions_body.push_str("\n}\n");
      (format!("{}{}{}", action_functions_header,
                         field_declarations,
                         action_functions_body),
      new_argument_types,
      args_count)
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
            *current_string = format!("pub fn {}s () -> HashMap <String, HashMap <String, String>> {{\n  let mut {}_map : HashMap <String, HashMap <String,String>> = HashMap::new();\n", state, state);
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
    fn extract_p4_contents (&mut self, 
                            tokens : Vec<String>,
                            constants_list : Vec <String>) -> 
                            String {
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
      let mut tmp_token_buffer : Vec <String> = Vec::new(); 
      let mut types_to_instances_string : String = String::from("");
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
          if state == "table_actions" {
    
            state = String::from("none");
          }
        }
        if state == "ignore" {
          continue;
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
            if types_to_instances_string.len() == 0{
              types_to_instances_string.push_str(
                "// Returns HashMap mapping created instance name to the declared header_type or metadata name\n");
              types_to_instances_string.push_str(
                "pub fn types_to_instances () -> HashMap <String, String> { \n");
              types_to_instances_string.push_str(
                "  let mut types_to_instances_map : HashMap <String, String> = HashMap::new();\n");
            }
            types_to_instances_string.push_str(
                &format!("  types_to_instances_map.insert(String::from(\"{}\"), String::from(\"{}\"));\n",
                                                    &token,
                                                    &tokens[i+1]));
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
                 token == "control") &&
                 state != "comment" {
          state = String::from("ignore");
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
     if types_to_instances_string.len() > 0 {
       types_to_instances_string.push_str("  types_to_instances_map\n}\n");
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
     // Are there any registers? If so ensure they are returned
     // in the final HashMap. Otherise return an empty HashMap.
     // Repeat for counters, meters
     if register_stack.len() > 0 {
       for nn in register_stack.iter() {
         register_string.push_str(
            &format!("  register_map.insert(String::from(\"{}\"), {}_map);\n", nn, nn));
       }
       register_string.push_str("  register_map\n}\n");
     }
     else {
        register_string.push_str("pub fn registers () -> HashMap <String, HashMap <String, String>> {\n  HashMap::new()\n}\n "); 
     }

     if counter_stack.len() > 0 {
       for cn in counter_stack.iter() {
         counter_string.push_str(
            &format!("  counter_map.insert(String::from(\"{}\"), {}_map);\n ", cn, cn));
       }
       counter_string.push_str("  counter_map\n}\n");
     }
     else {
       counter_string.push_str("pub fn counters () -> HashMap <String, HashMap <String, String>> {\n  HashMap::new()\n}\n"); 

     }
     if meter_stack.len() > 0 {
       for mn in meter_stack.iter() {
         meter_string.push_str(
            &format!("  meter_map.insert(String::from(\"{}\"), {}_map);\n ", mn, mn));
       }
       meter_string.push_str("  meter_map\n}\n");

     }
     else {
       meter_string.push_str("pub fn meters () -> HashMap <String, HashMap <String, String>> {\n  HashMap::new()\n}\n"); 

     }

     let mut table_actions_types_map : HashMap <String, Vec<String>> = 
        HashMap::new();
     for ta in table_actions_list.iter() {
       table_actions_types_map.insert(ta.clone(), vec!["root".to_string()]);
     }

     let (action_functions_string, table_actions_to_num_args)  = 
                self.compound_actions_string(table_actions_list.clone(), 
                                             actions_to_strings_map , 
                                             table_actions_types_map,
                                             &constants_list);
     let caller_function_string : String = 
        match_action_generation_utils::generate_action_caller(table_actions_list, table_actions_to_num_args);
     format!("{}{}{}{}{}{}{}{}{}", action_functions_string,
                                   header_type_string,
                                   types_to_instances_string,
                                   table_matches_string,
                                   table_actions_string,
                                   counter_string,
                                   register_string,
                                   meter_string,
                                   caller_function_string)

    }
    // Opens P4 file and parses actions, matches, and headers.
    // Converts them into Rust code
    fn parse_p4_file (&mut self,
                      mut tokens : Vec<String>,
                      p4_files : Vec<String>,
                      constants_map : &mut HashMap <String, String>) -> String {

       let mut complete_p4_string : String = String::from("");
       for s in p4_files.iter() { 
         let (mut new_tokens, new_p4_files, new_constants_map) : 
            (Vec<String>, Vec<String>, HashMap<String, String>)
             = match_action_generation_utils::lexer(s.clone());
         tokens.append(&mut new_tokens);
         constants_map.extend(new_constants_map); 

         // NOTE: May need to process these new p4_files in future
       }
       let mut constants_list : Vec<String> = Vec::new();
       let constants_declaration : String = 
          match_action_generation_utils::generate_constant_declarations(&constants_map);
       for s in constants_map.keys() {
         constants_list.push(s.clone());
       }
       let file_string = self.extract_p4_contents(tokens,
                                                  constants_list);
       complete_p4_string.push_str(&file_string);
      
       format!("{}{}", constants_declaration, complete_p4_string)
    }


    pub fn generate (&mut self, 
                     schedule : HashMap <i32, Vec<String>>,
                     latencies_file : &str) {
      let (tokens, p4_files, mut constants_map) : (Vec<String>, Vec<String>, HashMap <String, String>) = 
        match_action_generation_utils::lexer(self.input_file.clone());
      let p4_string_data : String = self.parse_p4_file(tokens,
                                                       p4_files,
                                                       &mut constants_map);


      fs::write(self.output_file.to_string(), 
                format!("{}{}{}{}{}", match_action_generation_utils::generate_use_declarations(), 
                                  p4_string_data,
                                  match_action_generation_utils::generate_primitive_actions(),

                                  match_action_generation_utils::generate_hashmap_schedule(schedule),
                                  match_action_generation_utils::generate_latencies_functions(latencies_file))
                                ).expect("Error: could not write to output Rust file");
      println!("Output written to {}", self.output_file);
    }
}
