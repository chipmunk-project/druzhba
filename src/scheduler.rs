use crate::phv::Phv;
use std::env;
use std::fs;
use std::process::Command;

pub struct Scheduler {
    pub input_file : String,
}

impl Scheduler {


  pub fn exec_drmt_scheduler (&self) {
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

     let drmt_scheduler_output = Command::new("./run_drmt.sh")
                                 .arg(format!("{}", py_file))
                                 .arg("/home/mikewong/Documents/drmt")
                                 .output()
                                 .expect("Could not run DRMT scheduler");
/*
     let drmt_scheduler_output = Command::new("python")
                                 .arg("-u")
                                 .arg("../drmt/drmt.py")
                                 .arg(format!("{}", py_file))
                                 .arg("../drmt/large_hw")
                                 .arg("../drmt/drmt_latencies") 
                                 .arg("10")
                                 .output()
                                 .expect("Could not run DRMT scheduler");
*/
    println!("dRMT result: {}", 
              String::from_utf8_lossy(&drmt_scheduler_output.stdout));

  }
}

