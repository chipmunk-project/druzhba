
use std::fs;
use std::collections::HashMap;

pub fn generate_action_caller (table_actions : Vec<String>,
                               table_actions_to_num_args : HashMap <String, i32>) -> String
{
  let mut function_string = String::from("pub fn call_action (action : &str, args : Vec<i32>, pkt : &mut Packet, memories : &mut StatefulMemories) {\n");
  function_string.push_str("  match action {\n");
  // Generates a substring of the match expression
  let generate_string = | ta : &str, num_args : &i32| -> String {
    let mut match_string : String = format!("    \"{}\" => {}(", ta, ta);
    for i in 0..*num_args {
      match_string.push_str(&format!("args[{}], ",i));
    }

    match_string.push_str("pkt, memories),\n");
    match_string
  };
  
  for ta in table_actions.iter() {
    let num_args : &i32 = table_actions_to_num_args.get(ta)
                                                  .expect("Error: Could not find table action in generate_action_caller function");
    function_string.push_str(&generate_string(ta, 
                                              num_args));
  }
  function_string.push_str("    _ => panic!(\"Error: Invalid action provided to call_action function\"),\n  };\n}\n");
  function_string

}

pub fn generate_latencies_functions (latencies_file : &str) -> String {
  let mut getters_string : String = "".to_string();
  let latencies_file_contents : String = 
      fs::read_to_string(latencies_file.to_string())
              .expect("Error: could not read from latencies file");
  let latencies_lines : Vec<String> = latencies_file_contents
                                      .split("\n")
                                      .map(|s| s.to_string())
                                      .collect(); 
  for line in latencies_lines.iter() {
    let line_vec : Vec<String> = separate_by_whitespace(line.to_string());
    if line_vec.len() != 3 {
      continue;
    }
    // Must be in the form of value assignments. 
    // TODO: Consider if there are no spaces
//    assert!(line_vec.len() == 3);
    getters_string.push_str("pub fn get_");
    getters_string.push_str(
      match line_vec[0].as_str() {
        "dM" => "match_ticks () -> i32 {\n",
        "dA" => "action_ticks () -> i32 {\n",
        "dS" => "s () -> i32 {\n", // TODO: What is this?
        _    => panic!("Error: incorrect latency content"),
      }
   );
    getters_string.push_str(&format!("  {}\n}}\n", line_vec[2]));
  }
  getters_string
  

}
// Generates code to create hashmap of schedule
pub fn generate_hashmap_schedule (schedule : HashMap <i32, Vec<String>>) -> String {
    let mut generate_schedule_string : String = 
       String::from("pub fn generate_schedule() -> HashMap <i32, Vec<String>> {\n  let mut schedule : HashMap <i32, Vec<String>> = HashMap::new();\n");
    for (key, value) in schedule.into_iter() {
      if value.len() > 0 {
        let mut vec_string : String = String::from("");
        for v in value.iter() {
          vec_string.push_str(&format!("\"{}\".to_string(),", v));
        }
        generate_schedule_string.push_str(
         &format!("  schedule.insert({}, vec![{}]);\n", 
                                    key, vec_string) );
      }
    }
    generate_schedule_string.push_str("  schedule\n}\n");
    generate_schedule_string
}

pub fn generate_use_declarations () -> String {
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

pub fn generate_constant_declarations (constants_map : &HashMap <String, String>) -> String {
  let mut constant_declaration_string : String = String::from("");
  for v in constants_map.keys() {
    let value : String = constants_map.get(v)
        .expect("Error: Could not get constant value from HashMap").clone();
    constant_declaration_string.push_str(
      &format!("const {} : i32 = {};\n", v, value));
  }
  constant_declaration_string
}
fn generate_drop () -> String {
  "fn drop(p : &mut Packet) {\n  p.drop();\n}\n".to_string()
}
fn generate_modify_field () -> String {
  "fn modify_field (pkt_field : &mut i32, value : i32) {\n  *pkt_field = value;\n}\n".to_string()
}
fn generate_add_to_field () -> String {
  "fn add_to_field (pkt_field : &mut i32, value : i32) {\n  *pkt_field += value;\n}\n".to_string()    
}
fn generate_count () -> String {
  "fn count (c : &mut StatefulMemory, value : i32) {\n  c[value] += value; \n}\n".to_string()
}
pub fn generate_primitive_actions () -> String {
  format!("{}{}{}{}",
          generate_drop(),
          generate_modify_field(),
          generate_add_to_field(),
          generate_count())
}
// Takes in the included P4 file and modifies it so that
// the file can be found from the current directory
pub fn add_new_file (token : String,
                     complete_p4_files : &mut Vec<String>,
                     input_file : String) {
   // Strip the quotes
   let new_file : String = token[1..token.len() - 1].to_string();
   let path_vec : Vec <String> = input_file
                                 .split("/")
                                 .map (|s| s.to_string())
                                 .collect();
   if path_vec.len() == 1 {
     complete_p4_files.push(new_file);
   } 
   else {
     let mut altered_p4_file : String = String::from("");
     for j in 0..path_vec.len() - 1 {
       // If absolute path, prepend another /
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

fn separate_by_whitespace ( s : String) -> Vec <String> {
  s.split_whitespace()
   .into_iter()
   .map(|s| s.to_string())
   .collect()
}

 // Splits string using a given delimiter and returns
// a Vec<String>
fn split_string (current_string : String,
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
fn divide_into_vec (current_string : String,
                    delimiter : String) -> Vec<String> {

  let mut ret_vec : Vec<String> = Vec::new();
  let string_vec : Vec<String> = split_string(
                                     current_string.clone(),
                                     delimiter.clone());
  for i in 0..string_vec.len() {
    let s : String = string_vec[i].clone();
    let mut s_simplified = simplify_string(s);
    ret_vec.append(&mut s_simplified);
    if i < string_vec.len() - 1{
      ret_vec.push(delimiter.clone());
    }
 
  }
  ret_vec
}
         
// Initiates string splitting if the string contains any 
// important tokens we would like to separate        
fn simplify_string (current_string : String) -> Vec<String> {

  if current_string.contains ("(") {
    divide_into_vec(current_string.clone(),
                         "(".to_string())
  }
  else if current_string.contains (")") {
    divide_into_vec(current_string.clone(),
                         ")".to_string())
  }
  else if current_string.contains ("{") {
    divide_into_vec(current_string.clone(),
                         "{".to_string())
  }
  else if current_string.contains ("}") {
    divide_into_vec(current_string.clone(),
                         "}".to_string())
  }
  else if current_string.contains (",") {
    divide_into_vec(current_string.clone(),
                         ",".to_string())
  }
  else if current_string.contains("/*"){
    divide_into_vec(current_string.clone(),
                         "/*".to_string())
  }
  else if current_string.contains("*/"){
    divide_into_vec(current_string.clone(),
                         "*/".to_string())
  }
  else if current_string.contains("//"){
    divide_into_vec(current_string.clone(),
                         "//".to_string())
  }
  else if current_string.contains(";"){
    divide_into_vec(current_string.clone(),
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
pub fn lexer (file : String) -> (Vec <String>,
                                 Vec<String>,
                                 HashMap <String, String>) { 
  let p4_file_contents : String = fs::read_to_string(file.clone())
                                  .expect("Error: could not read from P4 file");
  let p4_file_lines : Vec<String> = p4_file_contents
                                    .split("\n")
                                    .map(|s| s.to_string())
                                    .collect(); 
  let mut tokens : Vec<String> = Vec::new();
  let mut includes_files : Vec<String> = Vec::new();
  let mut constants_map : HashMap <String, String> = HashMap::new(); 
  // Iterate thorugh lines of P4 file
  for i in 0..p4_file_lines.len() {


    let line : String = p4_file_lines[i].clone();
    let line_vec : Vec<String> = 
       separate_by_whitespace(line);
    // Preprocessor interprets define and include macros
    // NOTE: macro functions are not interpreted
    if p4_file_lines[i].contains("#include") {
      add_new_file(line_vec[1].clone(), 
                   &mut includes_files, 
                   file.clone());
      continue;
    }
    if p4_file_lines[i].contains("#define"){

      let constant_value : Vec<String> = simplify_string(line_vec[2].clone()).clone();
      constants_map.insert(line_vec[1].clone(), 
                           constant_value[0].clone());
      continue;
    }
    for elem in line_vec.iter () {
      let mut elem_vec : Vec<String> = simplify_string(elem.to_string());
      if elem_vec.len() > 0 {
        tokens.append(&mut elem_vec);    
      }
      else {
        tokens.push(elem.to_string()); 
      }
    }
  }
  (tokens, includes_files, constants_map)
}

