use std::fs;
use std::process::Command;
use std::collections::HashMap;

pub struct Scheduler {
    pub input_file : String,
}

impl Scheduler {


  // Runs dRMT scheduler and parses output to get
  // matches/actions at certain cycles
  pub fn exec_drmt_scheduler (&self, path_to_drmt : &str) -> HashMap <i32, Vec<String> > {
     let output = Command::new("p4-graphs")
                  .arg("--split-match-action-events")
                  .arg(&self.input_file)
                  .output()
                  .expect("Could not generate DAG");
     println!("{}", String::from_utf8_lossy(&output.stdout));
     let file_path_string_vec : Vec <String> = self.input_file
                                               .split ("/")
                                               .map (|s| s.to_string())
                                               .collect();

     let mut py_file : String = 
        file_path_string_vec[file_path_string_vec.len() - 1].clone();

     py_file.truncate (py_file.len() - 1);
     let mut file_name : String = py_file.clone();
     file_name.truncate(file_name.len() - 2);
     py_file.push_str("y");
    

     println!("python file: {}", py_file);
     println!("Src: {}", format!("{}.ingress.sched_data.txt", file_name)); 
     Command::new("mv")
              .arg(format!("{}.ingress.sched_data.txt", file_name))
              .arg(format!("{}",py_file))
              .output()
              .expect("Could not change file name");

     assert!(path_to_drmt.chars().count() > 0 );
     let mut path_to_drmt_script : String = path_to_drmt.to_string();
     
     if path_to_drmt.chars().last().unwrap() == '/' {
       path_to_drmt_script.truncate(path_to_drmt_script.len() - 1);
     }
     path_to_drmt_script.trim();
       
     let drmt_scheduler_output = Command::new("./run_drmt.sh")
                                   .arg(format!("{}", py_file))
                                   .arg(path_to_drmt_script)
                                   .output()
                                   .expect("Could not run DRMT scheduler");
     
    let drmt_scheduler_result : String = 
      String::from_utf8_lossy (&drmt_scheduler_output.stdout)
      .to_string();
    println!("dRMT result: {}", drmt_scheduler_result);
    // Unfortuately dRMT scheduler sometimes prints
    // the '\n' escape sequence instead of a newline.
    // This if statement catches this behavior
    let corrected_drmt_scheduler_result : String = 
      str::replace(
        &str::replace(&drmt_scheduler_result, "|\\n|", "|\n|"),
        "|\\n\\n|", "|\n\n|").to_string();

     let drmt_scheduler_result_vec : Vec <String> = 
                              corrected_drmt_scheduler_result
                             .split ("\n")
                             .map (|s| s.to_string())
                             .collect();

    let mut cycles_to_matches_and_actions : HashMap <i32, Vec<String> >  = 
      HashMap::new();

        let mut hashmap_insertion_code : String = 
          "    let mut cycles_to_matches_and_actions : HashMap<i32, Vec<String> > = HashMap::new()\n".to_string();
    // Parse through dRMT result file and get matches/actions
    // and the cycles that they occur in
    for i in 0..drmt_scheduler_result_vec.len() {
      let mut line : &str = &drmt_scheduler_result_vec[i];
      // Enter the portion of the file that contains schedule
      if line.contains("scheduling period on one processor"){

        let mut offset : usize = 1;
        line = &drmt_scheduler_result_vec[i + offset];

        let mut temp_cycles : Vec <i32> = Vec::new();
        while !line.contains("Steady state"){

          // Contains a line in the schedule
          let line_vec : Vec <&str> = line
                                       .split("|")
                                       .map(|s| s.trim())
                                       .collect();
          // The scheduler prints a line of numerical cycles.
          // Below these values are the corresponding matches/actions
          // to be performed at that cycle.
          // First, get the numerical cycle values
          if line.contains("t="){
            temp_cycles.clear();
            for j in 0..line_vec.len() {
              if !&line_vec[j].contains("t="){
                continue;
              }
              let mut chars = line_vec[j].chars();
              chars.next();
              chars.next();
              let cycle = chars.as_str()
                               .to_string()
                               .parse::<i32>()  
                               .unwrap();
              if j >= temp_cycles.len() {
                temp_cycles.push (cycle);
                cycles_to_matches_and_actions
                  .insert(cycle, Vec::new());
              }
              else {
                temp_cycles[j] = cycle;
              }
              
            }
          }
          // Get the matches and actions that are used in
          // every cycle after the cycle values have been
          // found
          else {
            // First element is empty string
            for j in 0..line_vec.len(){
              let schedule_item : &str = &line_vec[j].trim();
              if schedule_item.contains("\\n\\n") || 
                 schedule_item == "" {
                continue;
              }
              let mut current_vec : Vec <String> = 
                cycles_to_matches_and_actions[&temp_cycles[*&j - 1]].clone();
              current_vec.push(schedule_item.to_string());
              cycles_to_matches_and_actions
                .insert(temp_cycles[*&j - 1], current_vec);
            }
          }
          offset += 1;
          line = &drmt_scheduler_result_vec[i + offset];
        }

      }
    }
    println!("{:?}", cycles_to_matches_and_actions);
    let insertion_code : &str = "    let cycles_to_matches_";
    cycles_to_matches_and_actions

  }
}

