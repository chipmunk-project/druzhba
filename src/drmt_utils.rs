
use druzhba::match_action::MatchAction;
use std::collections::HashMap;
use std::fs;
use druzhba::stateful_memory::{StatefulMemory,StatefulMemories};
use std::i32;
fn retrieve_memory_field_string (field : &str, 
                                 mem_data : &HashMap <String, String>) -> String {
  match mem_data.contains_key(field){
    true  =>  mem_data
                .get(field)
                .expect("Error: String field not in mem_data")
                .trim()
                .to_string(),
    false => "".to_string(),
  }
}
fn retrieve_memory_field_i32 (field : &str, 
                              mem_data : &HashMap <String, String>) -> i32 {
  match mem_data.contains_key(field){
    true  =>  mem_data
                .get(field)
                .expect("Error: i32 field not in mem_data")
                .trim()
                .parse::<i32>()
                .expect ("Error: i32 field could not be parsed"),

    false => 0,
  }
}
fn init_stateful_memory (stateful_memory_type : &str,
                         mem_data : &HashMap <String, String>) -> StatefulMemory {
  let stateful_memory_ic : i32 = retrieve_memory_field_i32("instance_count",
                                               mem_data);

  StatefulMemory {
    memory : stateful_memory_type.to_string(),
    memory_type : "".to_string(),   
    instance_count : retrieve_memory_field_i32("instance_count",
                                               mem_data),
    direct_or_static : retrieve_memory_field_string("direct_or_static",
                                                    mem_data),
    min_width : stateful_memory_ic,
    result : retrieve_memory_field_i32("result",
                                          mem_data),
    width : retrieve_memory_field_i32("width",
                                      mem_data),
    attributes : retrieve_memory_field_string("attributes",
                                              mem_data),
    memory_container : vec![0; stateful_memory_ic as usize]
  }

}
pub fn init_stateful_memories (registers : HashMap <String, HashMap <String, String>>,
                               counters : HashMap <String, HashMap <String, String>>,
                               meters : HashMap <String, HashMap<String, String>>) -> StatefulMemories {

  let mut stateful_memories_map : HashMap <String, StatefulMemory> = HashMap::new();
  for (reg_name, reg_data) in registers.iter() {
    stateful_memories_map.insert(reg_name.to_string(),
                                 init_stateful_memory("register",
                                 reg_data));
  }
  for (cnt_name, cnt_data) in counters.iter() {
    stateful_memories_map.insert(cnt_name.to_string(),
                                 init_stateful_memory("counter",
                                 cnt_data));
  }
 for (met_name, met_data) in meters.iter() {
    stateful_memories_map.insert(met_name.to_string(),
                          init_stateful_memory("meter",
                          met_data));
  }
  StatefulMemories {
    memories : stateful_memories_map 
  }
}
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

      if token == "{" {
        curly_brace_stack.push("{".to_string());
        if parsing_args {
          continue;
        }
      }
      else if token == "}" {
        curly_brace_stack.pop();
        if parsing_args {

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
