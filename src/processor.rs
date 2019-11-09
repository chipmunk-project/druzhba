use crate::phv::Phv;
use std::process::Command;

pub struct Processor {
    riscv_file : String,
    phvs : Vec<Phv<i32>>,
}

impl Processor {
    pub fn new () -> Processor {
        Processor {
            riscv_file : String::from(""),
            phvs : Vec::new(),
        }
    }

    pub fn with_riscv_file (t_riscv_file : String) 
        -> Processor {
         Processor {
            riscv_file : t_riscv_file,
            phvs : Vec::new(),
        }
       
    }
    pub fn add_phv (&mut self, input_phv : Phv<i32>)
    {
       self.phvs.push(input_phv); 
    }
    
    pub fn execute_program (&mut self) {
       let current_phv = self.phvs.remove(0);
       let mut binary_file = self.riscv_file.clone();
       binary_file.truncate(binary_file.len() - 2);

       Command::new("riscv64-unknown-elf-gcc")
                .arg(&self.riscv_file)
                .arg("-o")
                .arg(&binary_file)
                .output()
                .expect("Could not run cross compiler");

       let output = Command::new("spike")
                .arg("pk")
                .arg(&binary_file)
                .output()
                .expect("Could not run spike");
       println!("{}", String::from_utf8_lossy(&output.stdout));

       Command::new("rm")
                .arg(&binary_file)
                .output()
                .expect("Could not remove binary");
    }
}
