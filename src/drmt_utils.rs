
use druzhba::match_action::MatchAction;
use std::collections::HashMap;
use std::fs;
use std::i32;

pub fn parse_table_entries (table_entries_file : &str) 
                        -> HashMap <String, Vec<MatchAction>>
{
   
  let table_entry_file_contents : String = 
      fs::read_to_string(table_entries_file).expect ("Error: Table entries file could not be found");
  let table_entry_file_lines : Vec<String> = 
                                    table_entry_file_contents
                                    .split("\n")
                                    .map(|s| s.to_string())
                                    .collect(); 
  let mut table_name : String = "".to_string();
  let mut curly_brace_stack : Vec<String> = Vec::new();
  let mut match_action_map : HashMap <String, Vec<MatchAction>> = 
    HashMap::new();
  let mut fields_map : HashMap <String, String> = HashMap::new();
  let mut parsing_args : bool = false;
  let mut args_string = "".to_string();
  for line in table_entry_file_lines.iter() {
    // TODO: split line by whitespace
    let line_vec : Vec<String> = line
                                 .split_whitespace()
                                 .into_iter()
                                 .map(|s| s.to_string())
                                 .collect();
    for i in 0..line_vec.len() {
    
      let token : String = line_vec[i].clone();
      println!("Elem: {}", token); 

      if token == "{" {
        curly_brace_stack.push("{".to_string());
        if parsing_args {
          continue;
        }
      }
      else if token == "}" {
        curly_brace_stack.pop();
        if parsing_args {

          println!("parsing_args false");
          parsing_args = false;
        }
        if curly_brace_stack.len() == 0 {
          let match_action : MatchAction = 
            init_match_action(fields_map.clone(),
                              args_string.clone());
          if match_action_map.contains_key(&table_name) {
            match_action_map.get_mut(&table_name)
                            .expect("Error: match_action_map does not contain this table name")
                            .push(match_action);
          }
          else {
            match_action_map.insert(table_name.clone(),
                                    vec![match_action]);
          }
          table_name = "".to_string();
          fields_map.clear();
          args_string = "".to_string();
        }
      }
      if parsing_args {
        println!("args_string pushing {}", token);
        args_string.push_str(&token);
      }
      else if token == ":" &&
              line_vec[i-1] == "args" {
        println!("parsing_args true");
        parsing_args = true;
      }
      else if token == ":" &&
              line_vec[i-1] == "table" {
        table_name = format!("{}", line_vec[i+1]);
      }
      else if token == ":" {
        if line_vec[i-1] == "args" {
          parsing_args = true; 
        }
        else {
          fields_map.insert(line_vec[i-1].clone(),
                            line_vec[i+1].clone());
        }
      }
    }
  }
  match_action_map
}

fn init_match_action (fields_map : HashMap <String, String>,
                      args_string : String)
                      -> MatchAction {
  // Convert args to i32 and strip comments and whitespace
  println!("args string: {}", args_string);
  let args : Vec<i32> = args_string.split(",")
                                       .map(|s| {
                                         s.trim()
                                         .parse::<i32>()
                                         .expect ("Error: Expected i32 in action args") })
                                         
                                       .collect();        
  MatchAction {
      match_header_type : fields_map
                          .get("header_type")
                          .expect("Error: could not retrieve header_type")
                          .clone(),
      match_field : fields_map
                    .get("field")
                    .expect("Error: could not retrieve match field")
                    .clone(),
      match_type : fields_map
                    .get("type")
                    .expect("Error: could not retrieve match type")
                    .clone(),
      match_value : fields_map
                    .get("match_value")
                    .expect("Error: could not retrieve match type")
                    .trim()
                    .parse::<i32>()
                    .expect("Error: match_value cannot be parsed to i32"),
      range : fields_map
                    .get("range")
                    .expect("Error: could not retrieve range")
                    .trim()
                    .parse::<i32>()
                    .expect("Error: range cannot be parsed to i32"),
     mask : i32::from_str_radix(fields_map
                    .get("mask")
                    .expect("Error: could not retrieve mask")
                    .trim()
                    .trim_start_matches("0x"), 16).unwrap(),

      action_name : fields_map
                    .get("action_name")
                    .expect("Error: could not retrieve action name")
                    .clone(),
      action_args : args
              }

}
